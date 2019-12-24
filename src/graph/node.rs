use crate::plugin::Plugin;
use crate::utils::Id;

pub struct Node<'a> {
    plugin: &'a Plugin,
}

impl<'a> Node<'a> {
    pub fn new(plugin: &'a Plugin) -> Self {
        Self { plugin }
    }
}
