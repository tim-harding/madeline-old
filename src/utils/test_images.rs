use crate::image::{self, Image};
use crate::utils::Vec2I;

pub fn checker(size: Vec2I) -> Image {
    let desc = image::Desc::new(size, 4);
    let mut out = Image::from_desc(desc);
    for (y, line) in out.lines_mut().enumerate() {
        for (x, pixel) in line.enumerate() {
            let y_check = y % 32 < 16;
            let x_check = x % 32 < 16;
            let check = y_check ^ x_check;
            let remap = (check as usize as f32) * 0.5 + 0.25;

            pixel[0] = remap;
            pixel[1] = remap;
            pixel[2] = remap;
            pixel[3] = 1.0;
        }
    }
    out
}

pub fn solid_color(size: Vec2I, r: f32, g: f32, b: f32) -> Image {
    let desc = image::Desc::new(size, 4);
    let mut out = Image::from_desc(desc);
    for line in out.lines_mut() {
        for pixel in line {
            pixel[0] = r;
            pixel[1] = g;
            pixel[2] = b;
            pixel[3] = 1.0;
        }
    }
    out
}
