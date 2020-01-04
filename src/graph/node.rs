use crate::utils::Id;

#[derive(Copy, Clone, Debug)]
pub struct Node {
    pub plugin: Id,
    pub dirty: bool,
}

impl Node {
    pub fn new(plugin: Id) -> Self {
        Self {
            plugin,
            dirty: true,
        }
    }
}
