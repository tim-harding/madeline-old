use super::plugin::{Control, Plugin, PluginId};
use crate::image::{self, Image};
use crate::utils::Vector2;
use std::collections::HashMap;

pub type NodeId = usize;

pub struct Node {
    pub plugin: PluginId,
    pub position: Vector2,
    pub dirty: bool,
    pub inputs: Vec<NodeId>,
    pub cache: Option<Image>,
    pub controls: Vec<Control>,
}

impl Node {
    pub fn new(plugin: &Plugin, plugin_id: PluginId) -> Self {
        Self {
            plugin: plugin_id,
            position: Vector2::default(),
            dirty: true,
            inputs: Vec::new(),
            cache: None,
            controls: plugin.controls(),
        }
    }

    pub fn refresh_cache(&mut self, desc: image::Description) {
        let new = Image::from_description(desc);
        self.cache = Some(new);
        self.dirty = true;
    }
}

pub struct Nodes {
    next_index: NodeId,
    nodes: HashMap<NodeId, Node>,
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

    pub fn with_id(&self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }
}
