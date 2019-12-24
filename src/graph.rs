mod node;

struct Connection {
    pub from: node::Id,
    pub to: node::Id,
}

impl Connection {
    pub fn new(from: node::Id, to: node::Id) -> Self {
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
