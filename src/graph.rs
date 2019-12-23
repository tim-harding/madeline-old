pub mod node;
pub mod plugin;

pub use node::{Node, Nodes};
pub use plugin::{Plugin, Plugins};

pub struct Graph {
    pub nodes: Nodes,
    pub plugins: Plugins,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Nodes::new(),
            plugins: Plugins::new(),
        }
    }
}
