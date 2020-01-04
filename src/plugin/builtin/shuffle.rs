use crate::control;
use crate::image::{Channel, Image};
use crate::plugin::{self, *};

enum Parameters {
    R,
    G,
    B,
    A,
}

const NAME: &str = "shuffle";
const INPUTS: [&str; 1] = ["bg"];
const CONTROLS: [control::Desc; 4] = [
    control::Desc::new("r", control::Kind::Integer),
    control::Desc::new("g", control::Kind::Integer),
    control::Desc::new("b", control::Kind::Integer),
    control::Desc::new("a", control::Kind::Integer),
];
const DESC: plugin::Desc = plugin::Desc::new(NAME, &INPUTS, &CONTROLS);

#[derive(Debug, Default)]
pub struct Shuffle {}

impl Plugin for Shuffle {
    fn render(&self, inputs: Inputs, controls: Controls) -> Result<Image, String> {
        let bg = match inputs[0] {
            Some(bg) => bg,
            None => return Err("Invalid background input".to_string()),
        };

        let mut out = Image::default();
        let remap = [
            controls[Parameters::R as usize].as_int(),
            controls[Parameters::G as usize].as_int(),
            controls[Parameters::B as usize].as_int(),
            controls[Parameters::A as usize].as_int(),
        ];

        for remap in remap.iter() {
            let channel = match bg.channels().nth(*remap) {
                Some(channel) => channel.clone(),
                None => Channel::new(bg.desc().size),
            };
            out.push(channel);
        }
        Ok(out)
    }

    fn desc(&self) -> &'static plugin::Desc {
        &DESC
    }
}
