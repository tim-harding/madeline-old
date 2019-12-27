use crate::plugin::Plugin;

#[derive(Copy, Clone)]
pub struct Node<'a> {
    plugin: &'a dyn Plugin,
}

impl<'a> Node<'a> {
    pub fn new(plugin: &'a dyn Plugin) -> Self {
        Self { plugin }
    }
}
