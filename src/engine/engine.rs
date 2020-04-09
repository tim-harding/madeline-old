use crate::{
    graph::{Graph, Node},
    image::Image,
    plugin::{self, Plugin},
    utils::{Id, Value},
};
use std::collections::HashMap;

#[derive(Default)]
pub struct Engine {
    next_id: Id,
    pub viewing: Id,

    pub plugins: HashMap<Id, Plugin>,
    pub plugin_names: HashMap<String, Id>,

    pub nodes: HashMap<Id, Node>,
    pub node_names: HashMap<String, Id>,

    pub graph: Graph,
    pub controls: HashMap<Id, Vec<Value>>,
    pub images: HashMap<Id, Image>,
}

impl Engine {
    pub fn new() -> Self {
        let mut default = Self::default();
        plugin::populate_builtin(&mut default.plugins, &mut default.plugin_names);
        default
    }

    pub fn insert_node(&mut self, node: Node, name: String) -> Id {
        let id = self.next_id;
        self.next_id += 1;
        self.images.insert(id, Default::default());
        self.nodes.insert(id, node);
        if let Some(plugin) = self.plugins.get(&node.plugin) {
            let desc = plugin.desc();
            self.graph.insert_node();
            self.controls.insert(id, desc.controls());
            self.node_names.insert(name, id);
        }
        id
    }

    pub fn delete_node(&mut self, id: Id) {
        self.images.remove(&id);
        self.nodes.remove(&id);
        self.graph.delete_node(id);
        self.controls.remove(&id);

        let to_remove = self
            .node_names
            .iter()
            .filter_map(|(k, v)| if *v == id { Some(k.clone()) } else { None })
            .next()
            .unwrap();
        self.node_names.remove(&to_remove);
    }

    pub fn render(&mut self) -> Result<&Image, String> {
        let queue = self.graph.render_queue(self.viewing);
        for id in queue.iter().rev() {
            let node = self.nodes.get(id).ok_or("Node not found")?;
            if !node.dirty {
                continue;
            }
            let plugin = self
                .plugins
                .get(&node.plugin)
                .ok_or("Plugin not found")?;
            let controls = self.controls.get(id).ok_or("Controls not found")?;
            let inputs: Vec<_> = (0..plugin.desc().inputs_len())
                .flat_map(|input| {
                    self.graph
                        .input(*id, input as u8)
                        .map(|node| self.images.get(&node))
                })
                .collect();
            let render = plugin.render(inputs.as_slice(), controls.as_slice())?;
            self.images.entry(*id).and_modify(|e| *e = render);
        }
        self.images
            .get(&self.viewing)
            .ok_or_else(|| "Comp image not found".into())
    }
}
