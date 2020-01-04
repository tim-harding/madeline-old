mod node;
pub use node::Node;

use crate::utils::{Id, Table};

#[derive(Debug, Clone, Default)]
pub struct Graph(pub Table<Vec<Option<Id>>>);

impl Graph {
    pub fn insert_node(&mut self, inputs: usize) {
        self.0.insert(vec![None; inputs]);
    }

    pub fn delete_node(&mut self, id: Id) {
        self.0.delete(id);
    }

    pub fn connect(&mut self, from: Id, to: Id, input: usize, dfs: &mut Dfs) {
        if let Some(from) = self.0.get_mut(from) {
            from[input] = Some(to);
        }
        if dfs.has_cycle(from, self) {
            self.disconnect(from, input);
        }
    }

    pub fn disconnect(&mut self, from: Id, input: usize) {
        if let Some(from) = self.0.get_mut(from) {
            from[input] = None;
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Dfs {
    marked: Table<bool>,
    on_stack: Table<bool>,
    render_stack: Vec<Id>,
}

impl Dfs {
    pub fn insert_node(&mut self) {
        self.marked.insert(false);
        self.on_stack.insert(false);
    }

    pub fn delete_node(&mut self, id: Id) {
        self.marked.delete(id);
        self.on_stack.delete(id);
    }

    pub fn process_queue(&mut self, id: Id, graph: &Graph) {
        for marked in self.marked.rows_mut() {
            *marked = false;
        }
        self.render_stack.clear();
        self.dfs_render(id, graph);
    }

    pub fn render_queue(&self) -> &Vec<Id> {
        &self.render_stack
    }

    pub fn has_cycle(&mut self, id: Id, graph: &Graph) -> bool {
        for (marked, on_stack) in self.marked.rows_mut().zip(self.on_stack.rows_mut()) {
            *marked = false;
            *on_stack = false;
        }
        self.dfs_cycle(id, graph)
    }

    fn dfs_cycle(&mut self, id: Id, graph: &Graph) -> bool {
        self.on_stack.update(id, true);
        self.marked.update(id, true);
        if let Some(inputs) = graph.0.get_ref(id) {
            for input in inputs.iter() {
                if let Some(input) = input {
                    if !self.marked.get(*input).unwrap_or(false) {
                        self.dfs_cycle(*input, graph);
                    } else if self.on_stack.get(*input).unwrap_or(false) {
                        return true;
                    }
                }
            }
        }
        self.on_stack.update(id, false);
        false
    }

    fn dfs_render(&mut self, id: Id, graph: &Graph) {
        self.marked.update(id, true);
        if let Some(inputs) = graph.0.get_ref(id) {
            for input in inputs.iter() {
                if let Some(input) = input {
                    if !self.marked.get(*input).unwrap_or(false) {
                        self.dfs_render(*input, graph);
                    }
                }
            }
        }
        self.render_stack.push(id)
    }
}
