pub struct Channel {
    pub pixels: Vec<f32>,
}

impl Channel {
    pub fn new(x: usize, y: usize) -> Channel {
        let count = x * y;
        let mut pixels = Vec::with_capacity(count);
        pixels.resize(count, 0.0);
        Channel {
            pixels,
        }
    }
}
