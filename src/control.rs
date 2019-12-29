use crate::utils::Vec2I;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Kind {
    Integer,
    Float,
    Text,
    Vec2,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Desc {
    name: &'static str,
    kind: Kind,
}

impl Desc {
    pub const fn new(name: &'static str, kind: Kind) -> Self {
        Self { name, kind }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Control {
    Integer(usize),
    Float(f32),
    Text(String),
    Vec2(Vec2I),
}

impl From<&Kind> for Control {
    fn from(kind: &Kind) -> Self {
        match kind {
            Kind::Integer => Control::Integer(Default::default()),
            Kind::Float => Control::Float(Default::default()),
            Kind::Text => Control::Text(Default::default()),
            Kind::Vec2 => Control::Vec2(Default::default()),
        }
    }
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
