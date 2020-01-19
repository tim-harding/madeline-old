use crate::control;
use crate::image::{self, Image};
use crate::plugin::{self, *};
use crate::utils::{Value, Vec2U};
use std::cmp::{min, max};

enum Parameters {
    SizeX,
    SizeY,
}

pub fn create() -> Plugin {
    let controls = [
        control::Desc::new("sx", Value::Integer(512)),
        control::Desc::new("sy", Value::Integer(512)),
    ];
    let desc = plugin::Desc::new("resize", &["bg"], &controls);
    Plugin::new(render, desc)
}

fn render(inputs: Inputs, controls: Controls) -> Result<Image, String> {
    let bg = match inputs[0] {
        Some(bg) => bg,
        None => return Err(String::from("Invalid background input")),
    };

    let sx = controls[Parameters::SizeX as usize].as_uint();
    let sy = controls[Parameters::SizeY as usize].as_uint();

    let h_buf = upscale_axis(&bg, sx);

    /*
    let h_buf = downscale_axis(&bg, sx);
    let v_buf = downscale_axis(&h_buf, sy);
    Ok(v_buf)
    */
    Ok(h_buf)
}

fn upscale_axis(src: &Image, dim: usize) -> Image {
    let scale_factor = src.desc().size.x as f32 / dim as f32;
    let buf_desc = image::Desc::new(Vec2U::new(dim, src.desc().size.y), src.channel_count());
    let mut buf = Image::from_desc(buf_desc); 
    for (src_channel, dst_channel) in src.channels().zip(buf.channels_mut()) {
        for (y, line) in dst_channel.lines_mut().enumerate() {
            for (x, px) in line.enumerate() {
                let pos = x as f32 * scale_factor;

                let base_x = pos as usize;

                let idxn = max(0, pos as isize - 1) as usize;
                let idx0 = min(src.desc().size.x - 1, base_x + 0);
                let idx1 = min(src.desc().size.x - 1, base_x + 1);
                let idx2 = min(src.desc().size.x - 1, base_x + 2);

                let idxy = y * src.desc().size.x;

                let y0 = src_channel[idxn + idxy];
                let y1 = src_channel[idx0 + idxy];
                let y2 = src_channel[idx1 + idxy];
                let y3 = src_channel[idx2 + idxy];

                let a0 = -0.5 * y0 + 1.5 * y1 - 1.5 * y2 + 0.5 * y3;
                let a1 = y0 - 2.5 * y1 + 2.0 * y2 - 0.5 * y3;
                let a2 = -0.5 * y0 + 0.5 * y2;
                let a3 = y1;

                let frac = pos.fract();
                let frac2 = frac * frac;

                *px = a0 * frac2 * frac + a1 * frac2 + a2 * frac + a3;
            }
        }
    }
    buf
}

fn downscale_axis(src: &Image, dim: usize) -> Image {
    let buf_desc = image::Desc::new(Vec2U::new(src.desc().size.y, dim), src.channel_count());
    let mut buf = Image::from_desc(buf_desc);

    let filter_width = 2.0f32;
    let scale_factor_x = src.desc().size.x as f32 / dim as f32;
    let offset_x = filter_width * scale_factor_x;

    for (src_channel, dst_channel) in src.channels().zip(buf.channels_mut()) {
        // Starting with x, which is out-of-order. However, since
        // dst is flipped over y=x, this yields in-order access to
        // the src buffer.
        for (x, line) in dst_channel.lines_mut().enumerate() {
            for (y, px) in line.enumerate() {
                let out_pos = x as f32 * scale_factor_x;
                let lo = (out_pos - offset_x).round() as isize;
                let hi = (out_pos + offset_x).round() as isize;

                let mut acc = 0.0;
                for i in lo..hi {
                    let x_index = min(src.desc().size.x - 1, max(0, i) as usize);
                    let index = y * src.desc().size.x + x_index;
                    let value = src_channel[index];
                    // Filter sampling should probably be done with relation
                    // to out_pos, in order to account for subpixel sampling
                    let filter = sample((i - lo) as f32, offset_x);
                    acc += value * filter;
                }

                *px = acc;
            }
        }
    }

    buf
}

fn sample(x: f32, radius: f32) -> f32 {
    gauss(1.0 - (x - radius).abs() / radius) / radius
}

fn gauss(x: f32) -> f32 {
    let rcp = 1.0 - x;
    rcp * x * x + x * (1.0 - rcp * rcp)
}
