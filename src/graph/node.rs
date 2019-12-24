use crate::plugin::Plugin;
use crate::utils::Id;
use std::collections::HashMap;

pub struct Node<'a> {
    plugin: &'a Plugin,
}

impl<'a> Node<'a> {
    // Whoever creates this should also create its controls
    pub fn new(plugin: &'a Plugin) -> Self {
        Self { plugin }
    }
}

pub struct Nodes<'a> {
    next_index: Id,
    nodes: HashMap<Id, Node<'a>>,
}

impl<'a> Nodes<'a> {
    pub fn new() -> Self {
        Self {
            next_index: 0,
            nodes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, node: Node<'a>) {
        self.next_index += 1;
        self.nodes.insert(self.next_index, node);
    }
}
