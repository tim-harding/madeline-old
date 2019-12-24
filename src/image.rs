mod channel;
pub use channel::{Channel, Color};

mod desc;
pub use desc::{Desc, Format};

use crate::utils::Vector2Int;

mod loading;

pub struct Image {
    size: Vector2Int,
    color: Color,
}

impl Image {
    pub fn new(size: Vector2Int, color: Color) -> Self {
        Self {
            size,
            color,
        }
    }

    pub fn width(&self) -> usize {
        self.size.x
    }

    pub fn height(&self) -> usize {
        self.size.y
    }

    pub fn channels(&self) -> &Color {
        &self.color
    }
}
