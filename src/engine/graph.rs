use crate::utils::Id;
use petgraph::{
    algo::is_cyclic_directed,
    graph::{EdgeIndex, NodeIndex},
    visit::EdgeRef,
};

#[derive(Copy, Clone, Debug)]
pub struct Node {
    pub plugin: Id,
    pub dirty: bool,
}

impl Node {
    pub fn new(plugin: Id) -> Self {
        Self {
            plugin,
            dirty: true,
        }
    }
}

type Input = u8;

#[derive(Debug, Clone, Default)]
pub struct Graph(petgraph::Graph<(), Input>);

impl Graph {
    pub fn insert_node(&mut self) {
        self.0.add_node(());
    }

    pub fn delete_node(&mut self, id: Id) {
        let index = NodeIndex::new(id);
        self.0.remove_node(index);
    }

    pub fn connect(&mut self, downstream: Id, upstream: Id, input: Input) {
        let upstream = NodeIndex::new(upstream);
        let downstream = NodeIndex::new(downstream);
        let edge = self.0.add_edge(downstream, upstream, input);
        if is_cyclic_directed(&self.0) {
            self.0.remove_edge(edge);
        }
    }

    pub fn disconnect(&mut self, from: Id, input: Input) {
        if let Some(input) = self.input(from, input) {
            let index = EdgeIndex::new(input);
            self.0.remove_edge(index);
        }
    }

    pub fn input(&self, node: Id, input: Input) -> Option<Id> {
        let index = NodeIndex::new(node);
        for edge in self.0.edges(index) {
            let index = edge.weight();
            if *index == input {
                return Some(edge.target().index());
            }
        }
        None
    }

    pub fn render_queue(&self, viewing: Id) -> Vec<Id> {
        let mut queue = Vec::new();
        self.render_queue_recurse(viewing, &mut queue);
        queue
    }

    fn render_queue_recurse(&self, node: Id, queue: &mut Vec<Id>) {
        queue.push(node);
        let index = NodeIndex::new(node);
        for edge in self.0.edges(index) {
            let index = edge.target().index();
            self.render_queue_recurse(index, queue);
        }
    }
}
