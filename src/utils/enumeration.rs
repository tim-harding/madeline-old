use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Enumeration {
    values: HashMap<String, usize>,
}

impl Enumeration {
    pub fn new<'a, I>(names: I) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        let mut values = HashMap::new();
        for (i, name) in names.enumerate() {
            values.insert((*name).into(), i);
        }
        Self { values }
    }

    pub fn index(&self, key: &str) -> Option<usize> {
        self.values.get(key).copied()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
