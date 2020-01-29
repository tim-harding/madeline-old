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
        Some(bg) => bg,
        None => return Err(String::from("Invalid background input")),
    };

    let mut desc = bg.desc();
    desc.channels = 0;

    let size: usize = controls[Parameters::Size as usize].as_uint();
    let mut filter = Vec::with_capacity(size);
    let samples = 1 + size * 2;
    for i in 0..samples {
        let value = sample(i, size);
        filter.push(value);
    }

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
    let mut out = Channel::new(flipped);
    let size = (filter.len() / 2 - 1) as isize;
    for y in 0..channel.size().y {
        for x in 0..channel.size().x {
            let mut acc = 0.0;
            for (i, cell) in filter.iter().enumerate() {
                let sample_x = x as isize + i as isize - size;
                let sample_x = min(max_dim, max(0, sample_x)) as usize;
                let index = y * channel.size().x + sample_x;
                let sample = channel.raw()[index];
                acc += sample * cell;
            }
            let out_index = x * channel.size().y + y;
            out[out_index] = acc;
        }
    }
    out
}

fn sample(i: usize, size: usize) -> f32 {
    let size = size as f32;
    let local = i as f32 - size;
    gauss(1.0 - local.abs() / size) / size
}

fn gauss(x: f32) -> f32 {
    let rcp = 1.0 - x;
    rcp * x * x + x * (1.0 - rcp * rcp)
}
