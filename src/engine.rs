use crate::{
    graph::{Dfs, Graph, Node},
    image::Image,
    plugin::{self, Plugin},
    utils::{Id, Table, Value},
};
use std::collections::HashMap;

#[derive(Default)]
pub struct Engine {
    pub viewing: usize,

    pub plugins: Table<Plugin>,
    // Consider whether names should be stored only here, as with nodes
    pub plugin_names: HashMap<String, Id>,

    pub nodes: Table<Node>,
    pub node_names: HashMap<String, Id>,

    pub graph: Graph,
    pub dfs: Dfs,
    pub controls: Table<Vec<Value>>,
    pub images: Table<Image>,
}

impl Engine {
    pub fn new() -> Self {
        let mut default = Self::default();
        plugin::populate_builtin(&mut default.plugins, &mut default.plugin_names);
        default
    }

    pub fn insert_node(&mut self, node: Node, name: String) -> Id {
        let id = self.images.insert(Default::default());
        self.dfs.insert_node();
        self.nodes.insert(node);
        if let Some(plugin) = self.plugins.get_ref(node.plugin) {
            let desc = plugin.desc();
            self.graph.insert_node(desc.inputs_len());
            self.controls.insert(desc.controls());
            self.node_names.insert(name, id);
        }
        id
    }

    pub fn delete_node(&mut self, id: usize) {
        self.images.delete(id);
        self.dfs.delete_node(id);
        self.nodes.delete(id);
        self.graph.delete_node(id);
        self.controls.delete(id);

        let to_remove = self
            .node_names
            .iter()
            .filter_map(|(k, v)| if *v == id { Some(k.clone()) } else { None })
            .nth(0)
            .unwrap();
        self.node_names.remove(&to_remove);
    }

    pub fn render(&mut self) -> Result<&Image, String> {
        self.dfs.process_queue(self.viewing, &self.graph);
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
            .get_ref(self.viewing)
            .ok_or_else(|| "Comp image not found".into())
    }
}
