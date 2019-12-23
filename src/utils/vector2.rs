pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x, 
            y,
        } 
    }

    pub fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}
