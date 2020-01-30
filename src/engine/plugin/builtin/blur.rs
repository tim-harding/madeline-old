use crate::{
    control,
    image::{Channel, Image},
    plugin::{self, *},
    utils::{Value, Vec2U},
};
use rayon::prelude::*;
use std::cmp::{max, min};

enum Parameters {
    Size,
}

pub fn create() -> Plugin {
    let controls = [control::Desc::new("size", Value::Integer(0))];
    let desc = plugin::Desc::new("blur", &["bg"], &controls);
    Plugin::new(render, desc)
}

fn render(inputs: Inputs, controls: Controls) -> Result<Image, String> {
    let bg = match inputs[0] {
        Some(bg) => Ok(bg),
        None => Err(String::from("Invalid background input")),
    }?;

    let filter = {
        let size = controls[Parameters::Size as usize].as_uint();
        let f_size = size as f32;
        (0..1 + size * 2)
            .map(|i| {
                let x = i as f32 - f_size;
                let x = 1.0 - x.abs() / f_size;
                let rcp = 1.0 - x;
                let x = rcp * x * x + x * (1.0 - rcp * rcp);
                x / f_size
            })
            .collect::<Vec<_>>()
    };

    Ok(bg
        .par_channels()
        .map(|channel| {
            let tmp = blur_axis(&channel, &filter);
            blur_axis(&tmp, &filter)
        })
        .collect::<Image>())
}

fn blur_axis(channel: &Channel, filter: &[f32]) -> Channel {
    let max_dim = channel.size().x as isize - 1;
    let flipped = Vec2U::new(channel.size().y, channel.size().x);
    let mut out = Channel::black(flipped);
    let size = (filter.len() / 2 - 1) as isize;
    for y in 0..channel.size().y {
        for x in 0..channel.size().x {
            let out_index = x * channel.size().y + y;
            out[out_index] = filter.iter().enumerate().fold(0.0, |acc, (i, cell)| {
                let sample_x = x as isize + i as isize - size;
                let sample_x = min(max_dim, max(0, sample_x)) as usize;
                let index = y * channel.size().x + sample_x;
                let sample = channel[index];
                acc + sample * cell
            });
        }
    }
    out
}
