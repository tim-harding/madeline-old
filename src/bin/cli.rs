use madeline::{
    control::Control,
    engine::Engine,
    graph::Node,
    mdl,
    utils::{io, Vec2I},
};
use std::path::Path;

use std::collections::HashMap;

fn main() {
    match render() {
        Ok(_) => {}
        Err(e) => println!("{:?}", e),
    }
}

fn render() -> Result<(), String> {
    let mdl = std::fs::read_to_string("data/test_comp.mdl").map_err(|_| "".to_string())?;
    let content = madeline::mdl::parse(&mdl)?;
    println!("{:#?}", content);
    Ok(())
}

fn unpack(mdl: &[mdl::Node]) -> Result<(), String> {
    let engine = Engine::new();
    let mut nodes = HashMap::new();
    for def in mdl {
        let plugin = engine
            .plugins
            .r#where(|plug| plug.desc().name() == def.kind)
            .ok_or(format!("Could not resolve plugin: {}", def.kind))?;

        let id = nodes
            .insert(&def.name, Node::new(plugin))
            .ok_or("Could not insert node")?;
        for attr in def.attributes.iter() {
            /*
            for (i, desc) in engine
                .plugins
                .get_ref(plugin)
                .unwrap()
                .desc()
                .control_descs
                .iter()
                .enumerate()
            {
                let mut found = false;
                if attr.key == desc.name() {
                    engine.controls.get_mut(id).unwrap()[i] = match attr.value {
                        mdl::Value::Text(text) => {

                        },
                        mdl::Value::Number(value) => {

                        },
                        mdl::Value::Identifier(name) => {

                        },
                        mdl::Value::Vector(values) => {

                        },
                    };
                }
            }
            */
        }
    }
    Ok(())
}

fn test() -> Result<(), String> {
    let mut engine = Engine::new();

    let loader = engine
        .plugins
        .r#where(|plug| plug.desc().name() == "loader")
        .ok_or("Failed to find loader plugin")?;
    let merge = engine
        .plugins
        .r#where(|plug| plug.desc().name() == "merge")
        .ok_or("Failed to find merge plugin")?;
    let shuffle = engine
        .plugins
        .r#where(|plug| plug.desc().name() == "shuffle")
        .ok_or("Failed to find shuffle plugin")?;
    let blur = engine
        .plugins
        .r#where(|plug| plug.desc().name() == "blur")
        .ok_or("Failed to find blur plugin")?;

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
    engine.controls.get_mut(comp).ok_or("Comp not found")?[0] = Control::Vec2(Vec2I::new(200, 200));
    engine.graph.connect(comp, tree, 0, &mut engine.dfs);
    engine.graph.connect(comp, kitty, 1, &mut engine.dfs);

    let swizzle = engine.insert_node(Node::new(shuffle));
    engine.graph.connect(swizzle, comp, 0, &mut engine.dfs);
    engine
        .controls
        .get_mut(swizzle)
        .ok_or("Swizzle R not found")?[0] = Control::Integer(0);
    engine
        .controls
        .get_mut(swizzle)
        .ok_or("Swizzle G not found")?[1] = Control::Integer(0);
    engine
        .controls
        .get_mut(swizzle)
        .ok_or("Swizzle B not found")?[2] = Control::Integer(0);
    engine
        .controls
        .get_mut(swizzle)
        .ok_or("Swizzle A not found")?[3] = Control::Integer(3);

    let smear = engine.insert_node(Node::new(blur));
    engine.graph.connect(smear, swizzle, 0, &mut engine.dfs);
    engine
        .controls
        .get_mut(smear)
        .ok_or("Smear control not found")?[0] = Control::Integer(24);

    let comp = engine.render(smear)?;
    io::save(Path::new("data/merge.png"), comp)
}
