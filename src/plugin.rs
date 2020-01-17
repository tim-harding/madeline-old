mod plugins;
use crate::utils::Enumeration;
pub use plugins::*;

pub mod builtin;

use crate::control::{self, Control};
use crate::image::Image;
use crate::plugin;

pub type Inputs<'a> = &'a [Option<&'a Image>];
pub type Controls<'a> = &'a [Control];
pub type Render = fn(Inputs, Controls) -> Result<Image, String>;

#[derive(Clone, Debug)]
pub struct Desc {
    name: String,
    inputs: Enumeration,
    controls: Vec<Control>,
    controls_map: Enumeration,
}

impl Desc {
    pub fn new(name: &str, inputs: &[&str], controls: &[control::Desc]) -> Self {
        Self {
            name: name.into(),
            controls_map: Enumeration::new(controls.iter().map(|desc| desc.name)),
            controls: controls
                .iter()
                .map(|desc| desc.kind.clone())
                .collect::<Vec<_>>(),
            inputs: Enumeration::new(inputs.iter().copied()),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn index_for_control(&self, name: &str) -> Option<usize> {
        self.controls_map.index(name)
    }

    pub fn index_for_input(&self, name: &str) -> Option<usize> {
        self.inputs.index(name)
    }

    pub fn inputs_len(&self) -> usize {
        self.inputs.len()
    }

    pub fn controls(&self) -> Vec<Control> {
        self.controls.clone()
    }
}

pub struct Plugin {
    render: Render,
    desc: Desc,
}

impl Plugin {
    pub fn new(render: Render, desc: Desc) -> Self {
        Self { render, desc }
    }

    pub fn desc(&self) -> &Desc {
        &self.desc
    }

    pub fn render(&self, inputs: Inputs, controls: Controls) -> Result<Image, String> {
        (self.render)(inputs, controls)
    }
}
