use madeline::plugin::PluginsBuilder;
use madeline::utils::Table;
use madeline::graph::Node;

fn main() {
    let plugins = PluginsBuilder::builtin().build();
    let mut nodes: Table<Node> = Table::new();
}
