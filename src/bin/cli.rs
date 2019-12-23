use madeline::graph::Graph;
use madeline::plugins;

fn main() {
    let mut graph = Graph::new();
    graph.plugins.add(plugins::uv_texture::create());

    let image = graph.render();
}
