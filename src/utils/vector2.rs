#[derive(Eq, PartialEq, Default, Copy, Clone, Debug)]
pub struct Vec2Base<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2Base<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub type Vec2I = Vec2Base<usize>;
pub type Vec2 = Vec2Base<f32>;
