use crate::control;
use crate::image::{self, Image};
use crate::plugin::{self, *};
use crate::utils::{Value, Vec2U};
use std::cmp::max;

// TODO: options for edge-extension

enum Parameters {
    Left,
    Top,
    Right,
    Bottom,
}

pub fn create() -> Plugin {
    let controls = [
        control::Desc::new("left", Value::Integer(0)),
        control::Desc::new("top", Value::Integer(0)),
        control::Desc::new("right", Value::Integer(0)),
        control::Desc::new("bottom", Value::Integer(0)),
    ];
    let desc = plugin::Desc::new("crop", &["bg"], &controls);
    Plugin::new(render, desc)
}

fn render(inputs: Inputs, controls: Controls) -> Result<Image, String> {
    let bg = match inputs[0] {
        Some(bg) => bg,
        None => return Err(String::from("Invalid background input")),
    };

    let left = controls[Parameters::Left as usize].as_int();
    let top = controls[Parameters::Top as usize].as_int();
    let right = controls[Parameters::Right as usize].as_int();
    let bottom = controls[Parameters::Bottom as usize].as_int();

    let bottom = bg.desc().size.y as isize + bottom;
    let right = bg.desc().size.x as isize + right;

    let width = max(0, right + left) as usize;
    let height = max(0, bottom + top) as usize;

    let out_desc = image::Desc::new(Vec2U::new(width, height), bg.channel_count());
    let mut out = Image::from_desc(out_desc);

    let src_sz_x = bg.desc().size.x as isize;
    let src_sz_y = bg.desc().size.y as isize;
    for (src_channel, dst_channel) in bg.channels().zip(out.channels_mut()) {
        for (y, line) in dst_channel.lines_mut().enumerate() {
            for (x, px) in line.enumerate() {
                let src_x = x as isize - left;
                let src_y = y as isize - top;
                let src_idx = src_y * src_sz_x + src_x;
                let black = src_x < 0 || src_x > src_sz_x - 1 || src_y < 0 || src_y > src_sz_y - 1;
                let sample = if black {
                    0.0
                } else {
                    src_channel[src_idx as usize]
                };
                *px = sample;
            }
        }
    }

    Ok(out)
}
