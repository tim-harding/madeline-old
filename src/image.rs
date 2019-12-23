mod channel;
pub use channel::Channel;

mod description;
pub use description::Description;

mod loading;

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub channels: Vec<Channel>,
}

impl Image {
    pub fn from_desc(desc: Description) -> Image {
        let mut channels = Vec::with_capacity(desc.channels);
        for _ in 0..desc.channels {
            channels.push(Channel::new(desc.width, desc.height));
        }

        Image {
            width: desc.width,
            height: desc.height,
            channels,
        }
    }
}
