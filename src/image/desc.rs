use crate::utils::Vec2I;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Format {
    Grayscale,
    Rg,
    Rgb,
    Rgba,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Desc {
    size: Vec2I,
    format: Format,
}

impl Desc {
    pub fn new(size: Vec2I, format: Format) -> Self {
        Self { size, format }
    }

    pub fn size(&self) -> Vec2I {
        self.size
    }

    pub fn format(&self) -> Format {
        self.format
    }
}
