mod color;
pub use color::Color;

mod channels;
pub use channels::Channels;

use crate::utils::Vector2Int;

pub struct Channel {
    pub pixels: Vec<f32>,
}

impl Channel {
    pub fn new(size: Vector2Int) -> Channel {
        let count = size.x * size.y;
        let mut pixels = Vec::with_capacity(count);
        pixels.resize(count, 0.0);
        Channel { pixels }
    }
}
