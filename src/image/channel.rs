mod color;
pub use color::Color;

use crate::utils::Vec2I;

#[derive(Clone, Debug)]
pub struct Channel {
    pixels: Vec<f32>,
}

impl Channel {
    pub fn new(size: Vec2I) -> Channel {
        let count = size.x * size.y;
        let mut pixels = Vec::with_capacity(count);
        pixels.resize(count, 0.0);
        Channel { pixels }
    }

    pub fn set(&mut self, index: usize, value: f32) {
        self.pixels[index] = value
    }
}
