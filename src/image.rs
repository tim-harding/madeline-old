use crate::utils::Vec2I;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct ImageInfo {
    size: Vec2I,
    pixels: Vec<f32>,
}

impl Image {
    pub fn new(size: Vec2I) -> Self {
        let count = size.x * size.y * 4;
        let mut pixels = Vec::with_capacity(count);
        pixels.resize(count, 0);
        Self { size, pixels }
    }

    pub fn width(&self) -> usize {
        self.size.x
    }

    pub fn height(&self) -> usize {
        self.size.y
    }
}
