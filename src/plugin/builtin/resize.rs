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

    let h_buf = scale_axis(&bg, sx);
    let v_buf = scale_axis(&h_buf, sy);

    Ok(v_buf)
}

fn scale_axis(src: &Image, dim: usize) -> Image {
    if dim > src.desc().size.y {
        upscale_axis(src, dim)
    } else {
        downscale_axis(src, dim)
    }
}

fn upscale_axis(src: &Image, dim: usize) -> Image {
    let scale_factor = src.desc().size.x as f32 / dim as f32;
    let buf_desc = image::Desc::new(Vec2U::new(src.desc().size.y, dim), src.channel_count());
    let mut buf = Image::from_desc(buf_desc); 
    for (src_channel, dst_channel) in src.channels().zip(buf.channels_mut()) {
        for x in 0..buf_desc.size.x {
            for y in 0..buf_desc.size.y {
                let pos = y as f32 * scale_factor;

                let base_x = pos as usize;

                let idxn = max(0, pos as isize - 1) as usize;
                let idx0 = min(src.desc().size.x - 1, base_x + 0);
                let idx1 = min(src.desc().size.x - 1, base_x + 1);
                let idx2 = min(src.desc().size.x - 1, base_x + 2);

                let idxy = x * src.desc().size.x;

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

                let out_index = y * buf_desc.size.x + x;
                let px = a0 * frac2 * frac + a1 * frac2 + a2 * frac + a3;
                dst_channel[out_index] = px;
            }
        }
    }
    buf
}

fn downscale_axis(src: &Image, dim: usize) -> Image {
    let buf_desc = image::Desc::new(Vec2U::new(src.desc().size.y, dim), src.channel_count());
    let mut buf = Image::from_desc(buf_desc);

    let width = src.desc().size.x as f32 / dim as f32;

    for (src_channel, dst_channel) in src.channels().zip(buf.channels_mut()) {
        // Starting with x, which is out-of-order. However, since
        // dst is flipped over y=x, this yields in-order access to
        // the src buffer.
        for x in 0..buf_desc.size.x {
            for y in 0..buf_desc.size.y {
                let center = y as f32 * width;
                let lo = (center - width).floor() as isize;
                let hi = (center + width).ceil() as isize;

                let mut filter_acc = 0.0;
                let mut px_acc = 0.0;
                for i in lo..hi {
                    let rel = i as f32 - center;
                    let clipped = min(src.desc().size.x - 1, max(0, i) as usize);
                    let sample_index = x * src.desc().size.x + clipped;
                    let sample = src_channel[sample_index];

                    let filter = filter(rel, width);
                    px_acc += sample * filter;
                    filter_acc += filter;
                }

                let out_index = y * buf_desc.size.x + x;
                dst_channel[out_index] = px_acc / filter_acc;
            }
        }
    }

    buf
}

fn filter(src: f32, radius: f32) -> f32 {
    // Sawtooth
    let x = (1.0 - src.abs() / radius).clamp(0.0, 1.0);

    // Smoothstep
    let rcp = 1.0 - x;
    rcp * x * x + x * (1.0 - rcp * rcp)
}