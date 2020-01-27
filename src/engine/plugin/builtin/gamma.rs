use crate::{
    control,
    image::Image,
    plugin::{self, *},
    utils::Value,
};

enum Parameters {
    Gamma,
    Invert,
}

pub fn create() -> Plugin {
    let controls = [
        control::Desc::new("gamma", Value::Real(2.2)),
        control::Desc::new("invert", Value::Boolean(false)),
    ];
    let desc = plugin::Desc::new("gamma", &["bg"], &controls);
    Plugin::new(render, desc)
}

fn render(inputs: Inputs, controls: Controls) -> Result<Image, String> {
    let bg = match inputs[0] {
        Some(bg) => bg,
        None => return Err(String::from("Invalid background input")),
    };

    let mut out = bg.clone();

    let mut gamma = controls[Parameters::Gamma as usize].as_real();
    let invert = controls[Parameters::Invert as usize].as_bool();
    if invert {
        gamma = 1.0 / gamma;
    }

    for channel in out.channels_mut() {
        for element in channel.elements_mut() {
            *element = element.powf(gamma);
        }
    }

    Ok(out)
}
