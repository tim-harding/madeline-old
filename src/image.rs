mod channel;
pub use channel::Channel;

mod loading;

pub struct Desc {
    pub width: usize,
    pub height: usize,
    pub channels: usize,
}

impl Desc {
    pub fn new(width: usize, height: usize, channels: usize) -> Self {
        Self {
            width,
            height,
            channels,
        }
    }
}

pub struct Image {
    pub desc: Desc,
    channels: Vec<Channel>,
}

impl Image {
    pub fn from_desc(desc: Desc) -> Image {
        let mut channels = Vec::with_capacity(desc.channels);
        for _ in 0..desc.channels {
            channels.push(Channel::new(desc.width, desc.height));
        }

        Image { desc, channels }
    }
}
