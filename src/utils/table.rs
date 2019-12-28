use super::Id;
use std::slice::{Iter, IterMut};

pub struct Table<T> {
    next_id: Id,
    ids: Vec<Id>,
    values: Vec<T>,
}

impl<T> Table<T> {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            ids: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn add(&mut self, value: T) -> Id {
        self.next_id += 1;
        self.values.insert(self.next_id, value);
        self.next_id
    }

    pub fn remove(&mut self, id: Id) {
        match self.ids.as_slice().binary_search(&id) {
            Ok(index) => {
                self.ids.remove(index);
                self.values.remove(index);
            }
            Err(_) => {}
        }
    }

    pub fn get(&self, id: Id) -> Option<&T> {
        let value = self.values.get(id);
        match self.ids.as_slice().binary_search(&id) {
            Ok(index) => Some(&self.values[index]),
            Err(_) => None,
        }
    }

    pub fn get_mut(&mut self, id: Id) -> Option<&mut T> {
        let value = self.values.get(id);
        match self.ids.as_slice().binary_search(&id) {
            Ok(index) => Some(&mut self.values[index]),
            Err(_) => None,
        }
    }

    pub fn iter(&self) -> Iter<T> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.values.iter_mut()
    }
}
