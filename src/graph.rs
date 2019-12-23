pub mod node;
pub mod plugin;

pub use node::*;
pub use plugin::*;

pub struct Graph {
    nodes: Nodes,
    plugins: Plugins,
    viewing_node: NodeId,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Nodes::new(),
            plugins: Plugins::new(),
            viewing_node: std::usize::MAX,
        }
    }

    pub fn render(&mut self) {
        let mut stack = Vec::new();
        stack.push(self.viewing_node);
        
        while let Some(node_id) = stack.pop() {
            let node = match self.nodes.with_id(node_id) {
                Some(node) => node,
                None => continue,
            };
            stack.extend(node.inputs.iter());

            let plugin = self.plugins.with_id(node.plugin);
            let ctx = Context::new(node);

            let desc = plugin.image_description(ctx);
            let cache = match node.cache {
                Some(ref cache) => {
                    if cache.desc != desc {
                        node.refresh_cache(desc)
                    } else {
                        &mut cache
                    }
                },
                None => {
                    node.refresh_cache(desc)
                },
            };

            if node.dirty {
                plugin.render(ctx, &mut cache);
                node.dirty = false;
            }
        }
    }
}
