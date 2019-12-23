use crate::graph::{node::Node, plugin::control::Control};

pub struct Context<'a> {
    node: &'a Node,
}

impl<'a, 'b: 'a> Context<'a> {
    pub fn new(node: &'b Node) -> Self {
        Self {
            node,
        }
    }

    pub fn get_prop_int(&self, property: usize) -> usize {
        match self.node.controls[property] {
            Control::Integer(value) => value,
            _ => 0,
        }
    }

    pub fn get_prop_float(&self, property: usize) -> f32 {
        match self.node.controls[property] {
            Control::Float(value) => value,
            _ => 0.0,
        }
    }

    pub fn get_prop_text(&self, property: usize) -> &str {
        match self.node.controls[property] {
            Control::Text(ref value) => value.as_str(),
            _ => "",
        }
    }
}
