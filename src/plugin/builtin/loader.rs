use crate::control;
use crate::image::Image;
use crate::plugin::*;
use crate::utils::io;
use std::path::PathBuf;

const NAME: &str = "loader";
const INPUTS: [&str; 0] = [];
const CONTROLS: [control::Desc; 1] = [control::Desc::new("filename", control::Kind::Text)];
const DESC: plugin::Desc = plugin::Desc::new(NAME, &INPUTS, &CONTROLS);

enum Parameters {
    Filename,
}

#[derive(Debug, Default)]
pub struct Loader {}

impl Plugin for Loader {
    fn render(&self, _: Inputs, controls: Controls) -> Result<Image, String> {
        let path = PathBuf::from(controls[Parameters::Filename as usize].as_str());
        io::load(&path)
    }

    fn desc(&self) -> &'static Desc {
        &DESC
    }
}
