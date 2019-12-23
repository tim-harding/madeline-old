pub enum Kind {
    Integer,
    Float,
    Text,
}

pub struct Description {
    pub name: &'static str,
    pub kind: Kind,
}

impl Description {
    pub const fn new(name: &'static str, kind: Kind) -> Self {
        Self { name, kind }
    }
}

pub enum Control {
    Integer(usize),
    Float(f32),
    Text(String),
}

impl From<&Kind> for Control {
    fn from(kind: &Kind) -> Self {
        match kind {
            Kind::Integer => Control::Integer(0),
            Kind::Float => Control::Float(0.0),
            Kind::Text => Control::Text(String::new()),
        }
    }
}
