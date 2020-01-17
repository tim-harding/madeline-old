use madeline::{control::Control, engine::Engine, graph::Node, mdl, utils::io};

use std::collections::HashMap;
use std::path::Path;

fn main() {
    match render() {
        Ok(_) => {}
        Err(e) => println!("{:?}", e),
    }
}

fn render() -> Result<(), String> {
    let mdl = std::fs::read_to_string("data/test_comp.mdl").map_err(|_| "".to_string())?;
    let content = madeline::mdl::parse(&mdl)?;
    let mut engine = unpack(&content)?;
    let comp = engine.render()?;
    io::save(Path::new("data/merge.png"), comp)
}

fn unpack(mdl: &mdl::Graph) -> Result<Engine, String> {
    let mut engine = Engine::new();
    let mut nodes = HashMap::new();

    for def in mdl.nodes.iter() {
        let plugin_id = engine
            .plugins
            .r#where(|plug| plug.desc().name() == def.kind)
            .ok_or(format!("Could not resolve plugin: {}", def.kind))?;

        let id = engine.insert_node(Node::new(plugin_id));
        let plugin = engine.plugins.get_ref(plugin_id).unwrap();
        if let Some(_old) = nodes.insert(&def.name, id) {
            return Err(format!("Duplicate node name: {}", &def.name));
        }
        for attr in def.attributes.iter() {
            if let Some(index) = plugin.desc().index_for_control(&attr.key) {
                if let Some(controls) = engine.controls.get_mut(id) {
                    let new = match (&attr.value, &controls[index]) {
                        // TODO: Error reporting for non-matching types
                        (mdl::Value::Text(text), Control::Text(_)) => Control::Text(text.into()),
                        (mdl::Value::Real(value), Control::Real(_)) => Control::Real(*value),
                        (mdl::Value::Integer(value), Control::Integer(_)) => {
                            Control::Integer(*value)
                        }
                        _ => continue,
                    };
                    controls[index] = new;
                }
            }
        }
    }

    for def in mdl.nodes.iter() {
        let plugin_id = engine
            .plugins
            .r#where(|plug| plug.desc().name() == def.kind)
            .ok_or(format!("Could not resolve plugin: {}", def.kind))?;
        let plugin = engine.plugins.get_ref(plugin_id).unwrap();

        if let Some(downstream) = nodes.get(&def.name) {
            for pair in def.inputs.iter() {
                let input_name = &pair.key;
                let upstream_name = &pair.value;
                if let Some(input_index) = plugin.desc().index_for_input(&input_name) {
                    if let Some(upstream) = nodes.get(&upstream_name) {
                        engine
                            .graph
                            .connect(*downstream, *upstream, input_index, &mut engine.dfs);
                    }
                }
            }
        }
    }

    for option in mdl.options.iter() {
        match option.key.as_str() {
            "viewing" => {
                match &option.value {
                    mdl::Literal::Identifier(name) => {
                        match nodes.get(name) {
                            Some(id) => engine.viewing = *id,
                            _ => return Err("Could not resolve node name".into()),
                        }
                    },
                    _ => return Err("Viewing global should be an identifier".into()),
                }
            }
            _ => return Err("Unrecognized global option".into()),
        }
    }

    Ok(engine)
}