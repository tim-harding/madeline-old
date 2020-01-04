use madeline::control::Control;
use madeline::engine::Engine;
use madeline::graph::Node;
use madeline::utils::io;
use std::path::Path;

fn main() {
    match render() {
        Ok(_) => {}
        Err(e) => println!("{:?}", e),
    }
}

fn render() -> Result<(), String> {
    let mut engine = Engine::new();

    let loader = engine
        .plugins
        .r#where(|plug| plug.desc().name == "loader")
        .ok_or("Failed to find loader plugin")?;
    let merge = engine
        .plugins
        .r#where(|plug| plug.desc().name == "merge")
        .ok_or("Failed to find merge plugin")?;
    let shuffle = engine
        .plugins
        .r#where(|plug| plug.desc().name == "shuffle")
        .ok_or("Failed to find shuffle plugin")?;

    let kitty = engine.insert_node(Node::new(loader));
    engine
        .controls
        .get_mut(kitty)
        .ok_or("Loader controls not found")?[0] = Control::Text("data/kitty.png".to_string());

    let tree = engine.insert_node(Node::new(loader));
    engine
        .controls
        .get_mut(tree)
        .ok_or("Loader controls not found")?[0] = Control::Text("data/tree.png".to_string());

    let comp = engine.insert_node(Node::new(merge));
    engine.graph.connect(comp, tree, 0, &mut engine.dfs);
    engine.graph.connect(comp, kitty, 1, &mut engine.dfs);

    let swizzle = engine.insert_node(Node::new(shuffle));
    engine.graph.connect(swizzle, comp, 0, &mut engine.dfs);
    engine.controls.get_mut(swizzle).ok_or("Swizzle R not found")?[0] = Control::Integer(1);
    engine.controls.get_mut(swizzle).ok_or("Swizzle G not found")?[1] = Control::Integer(2);
    engine.controls.get_mut(swizzle).ok_or("Swizzle B not found")?[2] = Control::Integer(0);
    engine.controls.get_mut(swizzle).ok_or("Swizzle A not found")?[3] = Control::Integer(3);

    let comp = engine.render(swizzle)?;
    io::save(Path::new("data/merge.png"), comp)
}
