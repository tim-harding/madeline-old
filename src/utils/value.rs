use std::cmp::max;

#[derive(Debug, Clone)]
pub enum Value {
    Text(String),
    Real(f32),
    Integer(isize),
    Boolean(bool),
}

impl Value {
    pub fn as_int(&self) -> isize {
        match self {
            Value::Integer(value) => *value,
            _ => Default::default(),
        }
    }

    pub fn as_uint(&self) -> usize {
        match self {
            Value::Integer(value) => max(0, *value) as usize,
            _ => Default::default(),
        }
    }

    pub fn as_real(&self) -> f32 {
        match self {
            Value::Real(value) => *value,
            _ => Default::default(),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Boolean(value) => *value,
            _ => false,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Value::Text(value) => value.as_str(),
            _ => "",
        }
    }
}