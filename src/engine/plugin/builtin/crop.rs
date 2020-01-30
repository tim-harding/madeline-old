use crate::image::ChannelBuilder;
use crate::{
    control,
    image::Image,
    plugin::{self, *},
    utils::{Value, Vec2U},
};
use rayon::prelude::*;
use std::cmp::max;

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

    let src_sz_x = bg.desc().size.x as isize;
    let src_sz_y = bg.desc().size.y as isize;

    Ok(bg
        .par_channels()
        .map(|src_channel| {
            (0..height)
                .map(|y| {
                    (0..width).map(move |x| {
                        let src_x = x as isize - left;
                        let src_y = y as isize - top;
                        let src_idx = src_y * src_sz_x + src_x;
                        let black =
                            src_x < 0 || src_x > src_sz_x - 1 || src_y < 0 || src_y > src_sz_y - 1;
                        if black {
                            0.0
                        } else {
                            src_channel[src_idx as usize]
                        }
                    })
                })
                .flatten()
                .collect::<ChannelBuilder>()
                .build(Vec2U::new(width, height))
        })
        .collect::<Image>())
}
