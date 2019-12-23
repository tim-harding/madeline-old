pub mod node;
pub mod plugin;

pub use node::Node;
pub use plugin::{Plugin, Plugins};

pub struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }
    
    pub fn add(&mut self, node: Node) {
        self.nodes.push(node);
    }
}
