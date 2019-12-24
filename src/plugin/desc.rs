use crate::control;

type Name = &'static str;
type Inputs = &'static [&'static str];
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
