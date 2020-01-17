use crate::control::{self, Control};
use crate::image::{Channel, Image};
use crate::plugin::{self, *};

enum Parameters {
    R,
    G,
    B,
    A,
}

pub fn create() -> Plugin {
    let controls = [
        control::Desc::new("r", Control::Integer(0)),
        control::Desc::new("g", Control::Integer(1)),
        control::Desc::new("b", Control::Integer(2)),
        control::Desc::new("a", Control::Integer(3)),
    ];
    let desc = plugin::Desc::new("shufflel", &["bg"], &controls);
    Plugin::new(render, desc)
}

fn render(inputs: Inputs, controls: Controls) -> Result<Image, String> {
    let bg = match inputs[0] {
        Some(bg) => bg,
        None => return Err("Invalid background input".to_string()),
    };

    let mut out = Image::default();
    let remap = [
        controls[Parameters::R as usize].as_uint(),
        controls[Parameters::G as usize].as_uint(),
        controls[Parameters::B as usize].as_uint(),
        controls[Parameters::A as usize].as_uint(),
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
