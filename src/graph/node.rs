use crate::utils::Vector2;
use crate::image::Image;
use super::plugin::{
    Plugin,
    PluginId,
    Control,
};

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
