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

    pub fn render(&mut self, viewing: Id) -> Result<&Image, String> {
        self.dfs.process_queue(viewing, &self.graph);
        let queue = self.dfs.render_queue();
        for id in queue.iter() {
            let node = self.nodes.get(*id).ok_or("Node not found")?;
            if !node.dirty {
                continue;
            }
            let plugin = self
                .plugins
                .get_ref(node.plugin)
                .ok_or("Plugin not found")?;
            let controls = self.controls.get_ref(*id).ok_or("Controls not found")?;
            let inputs = self
                .graph
                .0
                .get_ref(*id)
                .map(|inputs| {
                    inputs
                        .iter()
                        .map(|maybe_id| {
                            maybe_id
                                .map(|input_id| self.images.get_ref(input_id))
                                .flatten()
                        })
                        .collect::<Vec<_>>()
                })
                .ok_or("Inputs not found")?;
            let render = plugin.render(inputs.as_slice(), controls.as_slice())?;
            self.images.update(*id, render);
        }
        self.images
            .get_ref(viewing)
            .ok_or("Comp image not found".to_string())
    }
}