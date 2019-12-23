pub struct Vector2Int {
    pub x: usize,
    pub y: usize,
}

impl Vector2Int {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn default() -> Self {
        Self::new(0, 0)
    }
}
