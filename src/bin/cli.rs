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

    engine.dfs.process_queue(comp, &engine.graph);
    let queue = engine.dfs.render_queue();
    for id in queue.iter() {
        let node = engine.nodes.get(*id).ok_or("Node not found.".to_string())?;
        if !node.dirty {
            continue;
        }
        let plugin = engine
            .plugins
            .get_ref(node.plugin)
            .ok_or("Plugin not found".to_string())?;
        let controls = engine
            .controls
            .get_ref(*id)
            .ok_or("Controls not found".to_string())?;
        let inputs = engine
            .graph
            .0
            .get_ref(*id)
            .map(|inputs| {
                inputs
                    .iter()
                    .map(|maybe_id| {
                        maybe_id
                            .map(|input_id| engine.images.get_ref(input_id))
                            .flatten()
                    })
                    .collect::<Vec<_>>()
            })
            .ok_or("Inputs not found".to_string())?;
        let render = plugin.render(inputs.as_slice(), controls.as_slice())?;
        engine.images.update(*id, render);
    }

    let comp = engine.images.get_ref(comp).ok_or("Comp image not found")?;
    io::save(Path::new("data/merge.png"), &comp)
}
