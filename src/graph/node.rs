use super::plugin::{Control, Plugin, PluginId};
use crate::image::Image;
use crate::utils::Vector2;
use std::collections::HashMap;

pub type NodeId = usize;

pub struct Node {
    pub plugin: PluginId,
    pub position: Vector2,
    pub inputs: Vec<NodeId>,
    pub cache: Option<Image>,
    pub controls: Vec<Control>,
}

impl Node {
    pub fn new(plugin: &Plugin, plugin_id: PluginId) -> Self {
        Self {
            plugin: plugin_id,
            position: Vector2::default(),
            inputs: Vec::new(),
            cache: None,
            controls: plugin.controls(),
        }
    }
}

pub struct Nodes {
    next_index: usize,
    nodes: HashMap<usize, Node>,
}

impl Nodes {
    pub fn new() -> Self {
        Self {
            next_index: 0,
            nodes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, node: Node) {
        self.next_index += 1;
        self.nodes.insert(self.next_index, node);
    }
}
