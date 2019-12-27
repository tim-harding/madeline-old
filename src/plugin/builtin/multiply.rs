use crate::control;
use crate::plugin::{self, *};
use crate::image::Image;

const NAME: &'static str = "Multiply";
const INPUTS: [&'static str; 2] = [
    "bg",
    "fg",
];
const CONTROLS: [control::Desc; 0] = [];
const DESC: plugin::Desc = plugin::Desc::new(NAME, &INPUTS, &CONTROLS);

pub struct Multiply { }

impl Multiply {
    pub fn new() -> Self {
        Self { }
    }
}

impl Plugin for Multiply {
    fn render(&self, inputs: Inputs) -> Result<Image, String> {
        let bg = match inputs[0] {
            Some(bg) => bg,
            None => return Err(String::from("Invalid background input")),
        };

        let fg = match inputs[1] {
            Some(fg) => fg,
            None => return Err(String::from("Invalid foreground input")),
        };
        
        if bg.desc() != fg.desc() {
            return Err(String::from("Non-matching inputs."));
        }

        let mut out = Image::from_desc(bg.desc());

        for ((px_bg, px_fg), px_out) in bg.pixels().zip(fg.pixels()).zip(out.pixels_mut()) {
            for ((channel_bg, channel_fg), channel_out) in px_bg.iter().zip(px_fg).zip(px_out) {
                *channel_out = *channel_fg * *channel_bg;
            }
        }

        Ok(out)
    }

    fn desc(&self) -> &'static plugin::Desc {
        &DESC
    }
}
