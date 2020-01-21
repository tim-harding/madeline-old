use super::*;
use crate::{
    engine::Engine,
    graph::Node,
};

pub fn apply(engine: &mut Engine, statement: &Statement) -> Result<(), String> {
    match statement {
        Statement::Assign { member, value } => {
            let node_id = match engine.node_names.get(&member.node) {
                Some(id) => Ok(id),
                None => Err("Node name not found".to_string()),
            }?;

            let plugin_id = match engine.nodes.get_ref(*node_id) {
                Some(node) => node.plugin,
                None => unreachable!(),
            };

            let control_index = match engine.plugins.get_ref(plugin_id) {
                Some(plugin) => plugin
                    .desc()
                    .index_for_control(&member.attr)
                    .ok_or("Attribute name not found".to_string()),
                None => unreachable!(),
            }?;

            match engine.controls.get_mut(*node_id) {
                Some(controls) => {
                    let control = controls[control_index];
                    match (control, value) {
                        (Value::Text(_), Value::Text(_)) => Ok(control = *value),
                        (Value::Boolean(_), Value::Boolean(_)) => Ok(control = *value),
                        (Value::Integer(_), Value::Integer(_)) => Ok(control = *value),
                        (Value::Real(_), Value::Real(_)) => Ok(control = *value),
                        (Value::Real(_), Value::Integer(int)) => {
                            Ok(control = Value::Real(*int as f32))
                        }
                        _ => Err("Attribute type does not match assignment".to_string()),
                    }
                }
                None => unreachable!(),
            }
        }

        Statement::New { kind, name } => {
            let plugin_id = match engine.plugin_names.get(kind) {
                Some(id) => Ok(id),
                None => Err("Node kind not found".to_string()),
            }?;
            let node = Node::new(*plugin_id);
            engine.insert_node(node, name.into());
            Ok(())
        }

        Statement::Delete { name } => {
            let id = match engine.node_names.get(name) {
                Some(id) => Ok(id),
                None => Err("Node name not found".to_string()),
            }?;
            engine.delete_node(*id);
            Ok(())
        }

        Statement::Glob { attr, value } => match attr.as_str() {
            "viewing" => match value {
                Literal::Identifier(name) => match engine.node_names.get(name) {
                    Some(id) => Ok(engine.viewing = *id),
                    None => Err("Node name not found".to_string()),
                },
                _ => Err("Viewing attribute takes a node identifier".to_string()),
            },
            _ => Err("Unrecognized global attribute".to_string()),
        },

        Statement::Wire {
            downstream,
            upstream,
        } => {
            let downstream_id = match engine.node_names.get(&downstream.node) {
                Some(id) => Ok(id),
                None => Err("Downstream node name not found".to_string()),
            }?;
            let upstream_id = match engine.node_names.get(upstream) {
                Some(id) => Ok(id),
                None => Err("Upstream node name not found".to_string()),
            }?;
            let downstream_node = match engine.nodes.get(*downstream_id) {
                Some(node) => node,
                None => unreachable!(),
            };
            let downstream_plugin = match engine.plugins.get_ref(downstream_node.plugin) {
                Some(plugin) => plugin,
                None => unreachable!(),
            };
            let input = match downstream_plugin.desc().index_for_input(&downstream.attr) {
                Some(index) => Ok(index),
                None => Err("Input name not found".to_string()),
            }?;
            engine
                .graph
                .connect(*downstream_id, *upstream_id, input, &mut engine.dfs);
            Ok(())
        }
    }
}
