use crate::utils::Vector2Int;

pub enum Format {
    Grayscale,
    Rg,
    Rgb,
    Rgba,
}

pub struct Desc {
    pub size: Vector2Int,
    pub format: Format,
}

impl Desc {
    pub fn new(size: Vector2Int, format: Format) -> Self {
        Self {
            size,
            format,
        }
    }
}

