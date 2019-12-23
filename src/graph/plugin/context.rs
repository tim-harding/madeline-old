use crate::graph::{
    Graph, 
    node::Node,
    plugin::control::Control,
};

pub struct Context<'a> {
    graph: &'a Graph,
    node: &'a Node,
}

impl Context<'_> {
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
