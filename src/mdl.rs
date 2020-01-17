use std::iter::Peekable;
use std::slice::Iter;

type Tokens<'a> = Peekable<Iter<'a, Token>>;

#[derive(Debug, Clone)]
pub struct Pair {
    pub key: String,
    pub value: Value,
}

impl Pair {
    pub fn new(key: String, value: Value) -> Self {
        Self { key, value }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Text(String),
    Number(f32),
    Identifier(String),
    Vector(Vec<f32>),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: String,
    pub name: String,
    pub inputs: Vec<Pair>,
    pub attributes: Vec<Pair>,
}

impl Node {
    pub fn new(kind: String, name: String, inputs: Vec<Pair>, attributes: Vec<Pair>) -> Self {
        Self { kind, name, inputs, attributes }
    }
}

#[derive(Debug, Clone)]
enum Token {
    Colon,
    Comma,
    ParenL,
    ParenR,
    CurlyL,
    CurlyR,
    SquareL,
    SquareR,
    Identifier(String),
    Text(String),
    Number(f32),
}

pub fn ast(src: &str) -> Result<Vec<Node>, String> {
    let tokens = tokens(src)?;
    let mut iter = tokens.iter().peekable();
    graph(&mut iter)
}

fn tokens(src: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut iter = src.chars().peekable();
    while let Some(c) = iter.next() {
        tokens.push(match c {
            ':' => Token::Colon,
            ',' => Token::Comma,
            '(' => Token::ParenL,
            ')' => Token::ParenR,
            '{' => Token::CurlyL,
            '}' => Token::CurlyR,
            '[' => Token::SquareL,
            ']' => Token::SquareR,
            '"' => {
                let mut value = String::new();
                while let Some(c) = iter.next() {
                    match c {
                        '"' => break,
                        other => value.push(other),
                    }
                }
                Token::Text(value)
            },
            other => {
                if other.is_ascii_alphabetic() {
                    let mut value = String::new();
                    value.push(other);
                    while let Some(c) = iter.peek() {
                        if c.is_ascii_alphabetic() {
                            value.push(iter.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    Token::Identifier(value)
                } else if other.is_ascii_digit() || other == '-' {
                    let mut value = String::new();
                    value.push(other);
                    while let Some(c) = iter.peek() {
                        if c.is_ascii_digit() || *c == '.' {
                            value.push(iter.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    match value.parse::<f32>() {
                        Ok(value) => Token::Number(value),
                        Err(_) => return Err("Could not parse number".into()),
                    }
                } else if other.is_ascii_whitespace() {
                    continue;
                } else {
                    return Err("Grammer does not match".into());
                }
            },
        });
    }
    Ok(tokens)
}

fn graph(iter: &mut Tokens) -> Result<Vec<Node>, String> {
    let mut nodes = Vec::new();
    while let Some(_) = iter.peek() {
        nodes.push(node(iter)?);
    }
    Ok(nodes)
}

fn node(iter: &mut Tokens) -> Result<Node, String> {
    let kind = match iter.next() {
        Some(Token::Identifier(name)) => name.into(),
        _ => return Err("Missing node kind".into()),
    };

    let name = match iter.next() {
        Some(Token::Identifier(name)) => name.into(),
        _ => return Err("Missing node name".into()),
    };

    match iter.next() {
        Some(Token::ParenL) => { },
        _ => return Err("Missing node inputs".into()),
    };

    let inputs = pairs(iter)?;

    match iter.next() {
        Some(Token::ParenR) => { },
        _ => return Err("Unclosed node inputs".into()),
    };

    match iter.next() {
        Some(Token::CurlyL) => { },
        _ => return Err("Missing node attributes".into()),
    };

    let attributes = pairs(iter)?;

    match iter.next() {
        Some(Token::CurlyR) => { },
        _ => return Err("Unclosed node attributes".into()),
    };

    Ok(Node::new(kind, name, inputs, attributes))
}

fn pairs(iter: &mut Tokens) -> Result<Vec<Pair>, String> {
    let mut pairs = Vec::new();
    while let Some(token) = iter.peek() {
        match token {
            Token::Identifier(_) => pairs.push(pair(iter)?),
            _ => break,
        }
    }
    Ok(pairs)
}

fn pair(iter: &mut Tokens) -> Result<Pair, String> {
    let key = match iter.next() {
        Some(Token::Identifier(name)) => name.into(),
        _ => return Err("Missing pair key".into()),
    };
    
    match iter.next() {
        Some(Token::Colon) => { },
        _ => return Err("Missing pair separator".into()),
    };

    let value = value(iter)?;
    
    match iter.peek() {
        Some(Token::Comma) => { iter.next(); },
        _ => { },
    }

    Ok(Pair::new(key, value))
}

fn value(iter: &mut Tokens) -> Result<Value, String> {
    match iter.next() {
        Some(token) => Ok(match token {
            Token::Text(value) => Value::Text(value.into()),
            Token::Number(value) => Value::Number(*value),
            Token::Identifier(value) => Value::Identifier(value.into()),
            Token::SquareL => {
                let mut values = Vec::new();
                while let Some(token) = iter.next() {
                    match token {
                        Token::Number(value) => {
                            values.push(*value);
                            match iter.next() {
                                Some(Token::Comma) => continue,
                                Some(Token::SquareR) => break,
                                _ => return Err("Invalid vector".into()),
                            }
                        },
                        _ => return Err("Invalid vector".into()),
                    }
                }
                Value::Vector(values)
            },
            _ => return Err("Invalid value".into()),
        }),
        _ => Err("Invalid value".into()),
    }
}