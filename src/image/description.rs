pub struct Description {
    pub width: usize,
    pub height: usize,
    pub channels: usize,
}

impl Description {
    pub fn new(width: usize, height: usize, channels: usize) -> Description {
        Description {
            width: width,
            height: height,
            channels: channels,
        }
    }
}
