use crate::utils::Value;

#[derive(Clone, Debug)]
pub struct Desc {
    pub name: &'static str,
    pub kind: Value,
}

impl Desc {
    pub fn new(name: &'static str, kind: Value) -> Self {
        Self { name, kind }
    }
}