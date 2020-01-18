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

    // This should later have the dimensions flipped to make memory access
    // for the next convolution stage linear
    let h_buf_desc = image::Desc::new(Vec2U::new(sx, bg.desc().size.y), bg.channel_count());
    let mut h_buf = Image::from_desc(h_buf_desc);

    let v_buf_desc = image::Desc::new(Vec2U::new(sx, sy), bg.channel_count());
    let mut _v_buf = Image::from_desc(v_buf_desc);

    let filter_width = 2.0f32;
    let scale_factor_x = bg.desc().size.x as f32 / sx as f32;
    let offset_x = filter_width * scale_factor_x;

    let _scale_factor_y = bg.desc().size.y as f32 / sy as f32;

    for (src_channel, dst_channel) in bg.channels().zip(h_buf.channels_mut()) {
        // If line weren't an iterator, we could just index into that
        // rather than calculating an index into the image.
        for (y, dst_line) in dst_channel.lines_mut().enumerate() {
            for (x, dst_px) in dst_line.enumerate() {
                let out_pos = x as f32 * scale_factor_x;
                let lo = (out_pos - offset_x).round() as isize;
                let hi = (out_pos + offset_x).round() as isize;

                let mut acc = 0.0;
                for i in lo..hi {
                    let x_index = min(bg.desc().size.x - 1, max(0, i) as usize);
                    let index = y * bg.desc().size.x + x_index;
                    let value = src_channel[index];
                    let filter = sample((i - lo) as f32, offset_x);
                    acc += value * filter;
                }
                *dst_px = acc;

                /*
                let x_index = min(bg.desc().size.x - 1, max(0, out_pos as isize) as usize);
                let index = y * bg.desc().size.x + x_index;
                let value = src_channel[index];
                *dst_px = value;
                */
            }
        }
    }

    // Should be v_buf in the end
    Ok(h_buf)
}

fn sample(x: f32, radius: f32) -> f32 {
    gauss(1.0 - (x - radius).abs() / radius) / radius
}

fn gauss(x: f32) -> f32 {
    let rcp = 1.0 - x;
    rcp * x * x + x * (1.0 - rcp * rcp)
}
