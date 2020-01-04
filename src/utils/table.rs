use super::Id;
use std::slice::{Iter, IterMut};

type Filter<T> = fn(value: &T) -> bool;

#[derive(Debug, Clone)]
pub struct Table<T> {
    next_id: Id,
    ids: Vec<Id>,
    values: Vec<T>,
}

impl<T> Default for Table<T> {
    fn default() -> Self {
        Self {
            next_id: 0,
            ids: Vec::new(),
            values: Vec::new(),
        }
    }
}

impl<T> Table<T> {
    pub fn insert(&mut self, value: T) -> Id {
        self.next_id += 1;
        self.ids.push(self.next_id);
        self.values.push(value);
        self.next_id
    }

    pub fn delete(&mut self, id: Id) {
        if let Ok(index) = self.ids.as_slice().binary_search(&id) {
            self.ids.remove(index);
            self.values.remove(index);
        }
    }

    pub fn update(&mut self, id: Id, value: T) {
        if let Some(row) = self.get_mut(id) {
            *row = value;
        }
    }

    pub fn r#where(&self, filter: Filter<T>) -> Option<Id> {
        self.rows().position(filter).map(|i| self.ids[i])
    }

    pub fn get_ref(&self, id: Id) -> Option<&T> {
        match self.ids.as_slice().binary_search(&id) {
            Ok(index) => Some(&self.values[index]),
            Err(_) => None,
        }
    }

    pub fn get_mut(&mut self, id: Id) -> Option<&mut T> {
        match self.ids.as_slice().binary_search(&id) {
            Ok(index) => Some(&mut self.values[index]),
            Err(_) => None,
        }
    }

    pub fn row_count(&self) -> usize {
        self.values.len()
    }

    pub fn rows(&self) -> Iter<T> {
        self.values.iter()
    }

    pub fn rows_mut(&mut self) -> IterMut<T> {
        self.values.iter_mut()
    }
}

impl<T> Table<T>
where
    T: Copy,
{
    pub fn get(&self, id: Id) -> Option<T> {
        self.get_ref(id).map(|v| *v)
    }
}
