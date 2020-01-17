use crate::utils::Enumeration;
use crate::utils::Vec2I;

#[derive(Clone, Debug)]
pub struct Desc {
    pub name: &'static str,
    pub enumeration: Option<Enumeration>,
    pub kind: Control,
}

impl Desc {
    pub fn new(name: &'static str, enumeration: Option<Enumeration>, kind: Control) -> Self {
        Self { name, enumeration, kind }
    }
}

#[derive(Clone, Debug)]
pub enum Control {
    Integer(usize),
    Float(f32),
    Text(String),
    Vec2(Vec2I),
}

impl Control {
    pub fn as_int(&self) -> usize {
        match self {
            Control::Integer(value) => *value,
            _ => Default::default(),
        }
    }

    pub fn as_float(&self) -> f32 {
        match self {
            Control::Float(value) => *value,
            _ => Default::default(),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Control::Text(value) => value.as_str(),
            _ => "",
        }
    }

    pub fn as_vec(&self) -> Vec2I {
        match self {
            Control::Vec2(value) => *value,
            _ => Default::default(),
        }
    }
}
