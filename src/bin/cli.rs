use madeline::graph::Plugins;
use madeline::plugins;

fn main() {
    let mut plugins = Plugins::new();
    plugins.add(plugins::uv_texture::create());
}
