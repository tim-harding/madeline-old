use crate::control;
use crate::image::Format;

type Name = &'static str;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Input {
    name: Name,
    format: Format,
}

impl Input {
    pub fn new(name: Name, format: Format) -> Self {
        Self { name, format }
    }

    pub fn name(&self) -> Name {
        self.name
    }

    pub fn format(&self) -> Format {
        self.format
    }
}

type Inputs = &'static [Input];
type Controls = &'static [control::Desc];

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Desc {
    name: Name,
    inputs: Inputs,
    controls: Controls,
}

impl Desc {
    pub const fn new(name: Name, inputs: Inputs, controls: Controls) -> Self {
        Self {
            name,
            inputs,
            controls,
        }
    }

    pub fn name(&self) -> Name {
        self.name
    }

    pub fn inputs(&self) -> Inputs {
        self.inputs
    }

    pub fn controls(&self) -> Controls {
        self.controls
    }
}
