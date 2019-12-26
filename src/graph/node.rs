use crate::plugin::Plugin;

#[derive(Debug, Copy, Clone)]
pub struct Node<'a> {
    plugin: &'a Plugin,
}

impl<'a> Node<'a> {
    pub fn new(plugin: &'a Plugin) -> Self {
        Self { plugin }
    }
}
