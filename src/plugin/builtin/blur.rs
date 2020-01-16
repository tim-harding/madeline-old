use crate::control;
use crate::image::{Channel, Image};
use crate::plugin::{self, *};
use crate::utils::Vec2U;
use std::cmp::{max, min};
use std::iter::repeat;

enum Parameters {
    Size,
}

const NAME: &str = "blur";
const INPUTS: [&str; 1] = ["bg"];
const CONTROLS: [control::Desc; 1] = [control::Desc::new("size", control::Kind::Integer)];
const DESC: plugin::Desc = plugin::Desc::new(NAME, &INPUTS, &CONTROLS);

#[derive(Debug, Default)]
pub struct Blur {}

impl Plugin for Blur {
    fn render(&self, inputs: Inputs, controls: Controls) -> Result<Image, String> {
        let bg = match inputs[0] {
            Some(bg) => bg,
            None => return Err(String::from("Invalid background input")),
        };

        let mut desc = bg.desc();
        desc.channels = 0;
        let mut out = Image::from_desc(desc);

        let flipped = Vec2U::new(desc.size.y, desc.size.x);

        let size = controls[Parameters::Size as usize].as_int();
        let mut filter = Vec::with_capacity(size);
        let samples = 1 + size * 2;
        for i in 0..samples {
            let value = sample(i, size);
            filter.push(value);
        }

        for channel in bg.channels() {
            let tmp = blur_axis(&channel, &filter);
            out.push(blur_axis(&tmp, &filter));
        }

        Ok(out)
    }

    fn desc(&self) -> &'static plugin::Desc {
        &DESC
    }
}

fn blur_axis(channel: &Channel, filter: &Vec<f32>) -> Channel {
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
