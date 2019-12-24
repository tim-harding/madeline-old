mod node;
use crate::utils::Id;

struct Connection {
    pub from: Id,
    pub to: Id,
}

impl Connection {
    pub fn new(from: Id, to: Id) -> Self {
        Self { from, to }
    }
}

pub struct Graph {
    connections: Vec<Connection>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
        }
    }
}
