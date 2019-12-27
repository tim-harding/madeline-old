use crate::control;
use crate::plugin::{self, *};
use crate::utils::Vec2I;
use crate::image::{self, Image};

const NAME: &'static str = "UV";
const INPUTS: [Input; 0] = [];
const CONTROLS: [control::Desc; 2] = [
    control::Desc::new("Width", control::Kind::Integer),
    control::Desc::new("Height", control::Kind::Integer),
];
const DESC: plugin::Desc = plugin::Desc::new(NAME, &INPUTS, &CONTROLS);

pub struct Uv { }

impl Uv {
    pub fn new() -> Self {
        Self { }
    }
}

impl Plugin for Uv {
    fn render(&self) -> Image {
        let size = Vec2I::new(1024, 1024);
        let desc = image::Desc::new(size, 2);
        let mut out = Image::from_desc(desc);
        let width = desc.size.x as f32;
        let height = desc.size.y as f32;
        for (y, line) in out.lines_mut().enumerate() {
            for (x, pixel) in line.enumerate() {
                pixel[0] = (x as f32) / width;
                pixel[1] = (y as f32) / height;
            }
        }
        out
    }

    fn desc(&self) -> &'static plugin::Desc {
        &DESC
    }
}
