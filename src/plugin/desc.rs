use crate::control;
use crate::image::Format;

pub struct Input {
    name: &'static str,
    format: Format,
}

type Name = &'static str;
type Inputs = &'static [Input];
type Controls = &'static [control::Desc];

pub struct Desc {
    pub name: Name,
    pub inputs: Inputs,
    pub controls: Controls,
}

impl Desc {
    pub const fn new(name: Name, inputs: Inputs, controls: Controls) -> Self {
        Self {
            name,
            inputs,
            controls,
        }
    }
}
