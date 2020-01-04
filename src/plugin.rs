mod plugins;
pub use plugins::*;

pub mod builtin;

use crate::control::{self, Control};
use crate::image::Image;
use crate::plugin;

type Name = &'static str;
type InputsDesc = &'static [&'static str];
type ControlsDesc = &'static [control::Desc];

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Desc {
    pub name: Name,
    pub inputs: InputsDesc,
    control_descs: ControlsDesc,
}

impl Desc {
    pub const fn new(name: Name, inputs: InputsDesc, controls: ControlsDesc) -> Self {
        Self {
            name,
            inputs,
            control_descs: controls,
        }
    }

    pub fn controls(&self) -> Vec<Control> {
        let mut out = Vec::new();
        for desc in self.control_descs.iter() {
            out.push(Control::from(&desc.kind()));
        }
        out
    }
}

pub type Inputs<'a> = &'a [Option<&'a Image>];
pub type Controls<'a> = &'a [Control];

pub trait Plugin {
    fn render(&self, inputs: Inputs, controls: Controls) -> Result<Image, String>;
    fn desc(&self) -> &'static plugin::Desc;
}
