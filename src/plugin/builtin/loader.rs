use crate::control::{self, Control};
use crate::image::Image;
use crate::plugin::*;
use crate::utils::io;
use std::path::PathBuf;

enum Parameters {
    Filename,
}

pub fn create() -> Plugin {
    let controls = [control::Desc::new("filename", Control::Text("".into()))];
    let desc = plugin::Desc::new("loader", &[], &controls);
    Plugin::new(render, desc)
}

fn render(_: Inputs, controls: Controls) -> Result<Image, String> {
    let path = PathBuf::from(controls[Parameters::Filename as usize].as_str());
    io::load(&path)
}
