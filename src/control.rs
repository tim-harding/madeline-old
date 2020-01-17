use std::cmp::max;

#[derive(Clone, Debug)]
pub struct Desc {
    pub name: &'static str,
    pub kind: Control,
}

impl Desc {
    pub fn new(name: &'static str, kind: Control) -> Self {
        Self { name, kind }
    }
}

#[derive(Clone, Debug)]
pub enum Control {
    Integer(isize),
    Real(f32),
    Text(String),
}

impl Control {
    pub fn as_int(&self) -> isize {
        match self {
            Control::Integer(value) => *value,
            _ => Default::default(),
        }
    }

    pub fn as_uint(&self) -> usize {
        match self {
            Control::Integer(value) => max(0, *value) as usize,
            _ => Default::default(),
        }
    }

    pub fn as_real(&self) -> f32 {
        match self {
            Control::Real(value) => *value,
            _ => Default::default(),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Control::Text(value) => value.as_str(),
            _ => "",
        }
    }
}
