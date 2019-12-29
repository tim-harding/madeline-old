use crate::control;
use crate::image::Image;
use crate::plugin::*;
use crate::utils::io;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::path::PathBuf;

const NAME: &'static str = "loader";
const INPUTS: [&'static str; 0] = [];
const CONTROLS: [control::Desc; 1] = [control::Desc::new("filename", control::Kind::Text)];
const DESC: plugin::Desc = plugin::Desc::new(NAME, &INPUTS, &CONTROLS);

enum Parameters {
    Filename,
}

pub struct Loader {
    images: HashMap<PathBuf, Image>,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
        }
    }
}

impl Plugin for Loader {
    fn render(&mut self, _: Inputs, controls: Controls) -> Result<Image, String> {
        let path = PathBuf::from(controls[Parameters::Filename as usize].as_str());
        match self.images.entry(path.clone()) {
            Occupied(entry) => Ok(entry.get().clone()),
            Vacant(_) => io::load(&path),
        }
    }

    fn desc(&self) -> &'static Desc {
        &DESC
    }
}
