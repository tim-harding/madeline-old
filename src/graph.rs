mod node;
pub use node::Node;

use crate::utils::Id;
use std::collections::{hash_map::Entry::*, hash_set, HashMap, HashSet};

#[derive(Debug, Default, Clone)]
pub struct Graph {
    inputs: HashMap<Id, HashSet<Id>>,
}

impl Graph {
    pub fn add(&mut self, from: Id, to: Id) {
        match self.inputs.entry(from) {
            Occupied(mut entry) => {
                entry.get_mut().insert(to);
            }
            Vacant(entry) => {
                let mut set = HashSet::new();
                set.insert(to);
                entry.insert(set);
            }
        };
    }

    pub fn remove(&mut self, from: Id, to: Id) {
        match self.inputs.entry(from) {
            Occupied(mut entry) => {
                entry.get_mut().remove(&to);
            }
            Vacant(_) => {}
        }
    }

    pub fn iter(&self, from: Id) -> Option<hash_set::Iter<Id>> {
        self.inputs.get(&from).map(|set| set.iter())
    }
}
