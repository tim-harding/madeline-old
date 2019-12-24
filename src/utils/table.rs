use super::Id;
use std::collections::HashMap;

pub struct Table<T> {
    next_id: Id,
    values: HashMap<Id, T>,
}

impl struct Table<T> {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            values: HashMap::new(),
        }
    }

    pub fn add(&mut self, value: T) -> Id {
        self.next_id += 1;
        self.values.insert(self.next_id, value);
        self.next_id
    }

    pub fn remove(&mut self, id: Id) {
        self.values.remove(&id)
    }

    pub fn get(&self, id: Id) -> &T {
        let value = self.values.get(&id);
        match value {
            Some(value) => value,
            None => unreachable!(),
        }
    }

    pub fn get_mut(&mut self, id: Id) -> &mut T {
        let value = self.values.get_mut(&id);
        match value {
            Some(value) => value,
            None => unreachable!(),
        }
    }
}
