use crate::control::{self, Control};

type Name = &'static str;

type Inputs = &'static [&'static str];
type Controls = &'static [control::Desc];

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Desc {
    pub name: Name,
    pub inputs: Inputs,
    control_descs: Controls,
}

impl Desc {
    pub const fn new(name: Name, inputs: Inputs, controls: Controls) -> Self {
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
