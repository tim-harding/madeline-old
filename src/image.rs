mod channel;
pub use channel::{Channel, Color};

mod desc;
pub use desc::{Desc, Format};

use crate::utils::Vec2I;

mod loading;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Image {
    size: Vec2I,
    color: Color,
}

impl Image {
    pub fn new(size: Vec2I, color: Color) -> Self {
        Self { size, color }
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
