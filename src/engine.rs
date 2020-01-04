use crate::control::Control;
use crate::graph::Dfs;
use crate::graph::Graph;
use crate::graph::Node;
use crate::image::Image;
use crate::plugin::{self, Plugins};
use crate::utils::Id;
use crate::utils::Table;

#[derive(Default)]
pub struct Engine {
    pub plugins: Plugins,
    pub nodes: Table<Node>,
    pub graph: Graph,
    pub dfs: Dfs,
    pub controls: Table<Vec<Control>>,
    pub images: Table<Image>,
}

impl Engine {
    pub fn new() -> Self {
        let mut default = Self::default();
        plugin::populate_builtin(&mut default.plugins);
        default
    }

    pub fn insert_node(&mut self, node: Node) -> Id {
        if let Some(plugin) = self.plugins.get_ref(node.plugin) {
            let desc = plugin.desc();
            self.graph.insert_node(desc.inputs.len());
            self.controls.insert(desc.controls());
        }
        self.images.insert(Default::default());
        self.dfs.insert_node();
        self.nodes.insert(node)
    }
}
