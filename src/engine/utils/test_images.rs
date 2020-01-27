use crate::{
    image::{self, Image},
    utils::Vec2U,
};

pub fn checker(size: Vec2U) -> Image {
    let desc = image::Desc::new(size, 4);
    let mut out = Image::from_desc(desc);
    for channel in out.channels_mut().take(3) {
        for (y, line) in channel.lines_mut().enumerate() {
            for (x, pixel) in line.enumerate() {
                let y_check = y % 32 < 16;
                let x_check = x % 32 < 16;
                let check = y_check ^ x_check;
                let remap = (check as usize as f32) * 0.5 + 0.25;
                *pixel = remap;
            }
        }
    }

    for element in out[3].elements_mut() {
        *element = 1.0;
    }

    out
}

pub fn solid_color(size: Vec2U, r: f32, g: f32, b: f32) -> Image {
    let desc = image::Desc::new(size, 4);
    let mut out = Image::from_desc(desc);
    let values = [r, g, b, 1.0];
    for (i, channel) in out.channels_mut().enumerate() {
        for element in channel.elements_mut() {
            *element = values[i];
        }
    }
    out
}
