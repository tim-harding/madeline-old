use super::*;
use crate::{engine::Engine, graph::Node};

pub fn apply(engine: &mut Engine, statement: &Statement) -> Result<(), String> {
    match statement {
        Statement::Assign { member, value } => {
            let node_id = match engine.node_names.get(&member.node) {
                Some(id) => Ok(id),
                None => Err(format!("Node name not found: {}", member.node)),
            }?;

            let plugin_id = match engine.nodes.get_ref(*node_id) {
                Some(node) => node.plugin,
                None => unreachable!(),
            };

            let control_index = match engine.plugins.get_ref(plugin_id) {
                Some(plugin) => plugin
                    .desc()
                    .index_for_control(&member.attr)
                    .ok_or(format!("Attribute name not found: {}", member.attr)),
                None => unreachable!(),
            }?;

            match engine.controls.get_mut(*node_id) {
                Some(controls) => {
                    let control = &controls[control_index];
                    controls[control_index] = match (control, value) {
                        (Value::Text(_), Value::Text(_)) => value.clone(),
                        (Value::Boolean(_), Value::Boolean(_)) => value.clone(),
                        (Value::Integer(_), Value::Integer(_)) => value.clone(),
                        (Value::Real(_), Value::Real(_)) => value.clone(),
                        (Value::Real(_), Value::Integer(int)) => Value::Real(*int as f32),
                        _ => {
                            return Err(format!(
                                "Attribute type does not match assignment: {}",
                                member
                            ))
                        }
                    };
                }
                None => unreachable!(),
            }
            Ok(())
        }

        Statement::New { kind, name } => {
            let plugin_id = match engine.plugin_names.get(kind) {
                Some(id) => Ok(id),
                None => Err(format!("Node kind not found: {}", kind)),
            }?;
            let node = Node::new(*plugin_id);
            engine.insert_node(node, name.into());
            Ok(())
        }

        Statement::Delete { name } => {
            let id = match engine.node_names.get(name) {
                Some(id) => Ok(*id),
                None => Err(format!("Node name not found: {}", name)),
            }?;
            engine.delete_node(id);
            Ok(())
        }

        Statement::Glob { attr, value } => match attr.as_str() {
            "viewing" => match value {
                Literal::Identifier(name) => match engine.node_names.get(name) {
                    Some(id) => {
                        engine.viewing = *id;
                        Ok(())
                    }
                    None => Err(format!("Node name not found: {}", name)),
                },
                _ => Err("Viewing attribute takes a node identifier".to_string()),
            },
            _ => Err(format!("Unrecognized global attribute: {}", attr)),
        },

        Statement::Wire {
            downstream,
            upstream,
        } => {
            let downstream_id = match engine.node_names.get(&downstream.node) {
                Some(id) => Ok(id),
                None => Err(format!(
                    "Downstream node name not found: {}",
                    downstream.node
                )),
            }?;
            let upstream_id = match engine.node_names.get(upstream) {
                Some(id) => Ok(id),
                None => Err(format!("Upstream node name not found: {}", upstream)),
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
                None => Err(format!("Input name not found: {}", downstream.attr)),
            }?;
            engine
                .graph
                .connect(*downstream_id, *upstream_id, input, &mut engine.dfs);
            Ok(())
        }
    }
}
