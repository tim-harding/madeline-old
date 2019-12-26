use madeline::graph::{Graph, Node};
use madeline::plugin::PluginsBuilder;
use madeline::utils::Table;

fn main() {
    let plugins = PluginsBuilder::builtin().build();
    let uv_plug = match plugins.get("UV") {
        Some(plug) => plug,
        None => unreachable!(),
    };

    let mut nodes: Table<Node> = Table::new();
    let uv_node = nodes.add(Node::new(uv_plug));

    let mut graph = Graph::new();
    graph.set_viewing(uv_node);
}
