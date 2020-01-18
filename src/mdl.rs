use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;
use crate::utils::Value;

mod unpack;
pub use unpack::*;

type Tokens<'a> = Peekable<Iter<'a, Token>>;

#[derive(Default, Debug, Clone)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub options: Vec<Pair<Literal>>,
}

#[derive(Debug, Clone)]
pub struct Pair<T> {
    pub key: String,
    pub value: T,
}

impl<T> Pair<T> {
    pub fn new(key: String, value: T) -> Self {
        Self { key, value }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    Value(Value),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: String,
    pub name: String,
    pub inputs: Vec<Pair<String>>,
    pub attributes: Vec<Pair<Value>>,
}

impl Node {
    pub fn new(
        kind: String,
        name: String,
        inputs: Vec<Pair<String>>,
        attributes: Vec<Pair<Value>>,
    ) -> Self {
        Self {
            kind,
            name,
            inputs,
            attributes,
        }
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

    Identifier(String),
    Text(String),
    Real(f32),
    Integer(isize),

    Glob,
    True,
    False,
}

pub fn parse(src: &str) -> Result<Graph, String> {
    let tokens = tokens(src)?;
    let mut iter = tokens.iter().peekable();
    graph(&mut iter)
}

fn tokens(src: &str) -> Result<Vec<Token>, String> {
    let mut keywords = HashMap::new();
    keywords.insert("glob", Token::Glob);
    keywords.insert("true", Token::True);
    keywords.insert("false", Token::False);

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
            '#' => {
                iter.find(|c| *c == '\n');
                continue;
            }
            '"' => {
                let mut value = String::new();
                while let Some(c) = iter.next() {
                    match c {
                        '"' => break,
                        other => value.push(other),
                    }
                }
                Token::Text(value)
            }
            other => {
                if other.is_ascii_alphabetic() {
                    let mut value = String::new();
                    value.push(other);
                    while let Some(c) = iter.peek() {
                        if c.is_ascii_alphabetic() || *c == '_' {
                            value.push(iter.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    match keywords.get(value.as_str()) {
                        Some(token) => token.clone(),
                        None => Token::Identifier(value),
                    }
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
                    match value.parse::<isize>() {
                        Ok(value) => Token::Integer(value),
                        Err(_) => match value.parse::<f32>() {
                            Ok(value) => Token::Real(value),
                            Err(_) => return Err("Could not parse number".into()),
                        },
                    }
                } else if other.is_ascii_whitespace() {
                    continue;
                } else {
                    return Err("Grammer does not match".into());
                }
            }
        });
    }
    Ok(tokens)
}

fn graph(iter: &mut Tokens) -> Result<Graph, String> {
    let mut graph = Graph::default();
    while let Some(token) = iter.peek() {
        match token {
            Token::Glob => {
                iter.next();
                graph.options.push(pair(iter, literal)?);
            }
            _ => graph.nodes.push(node(iter)?),
        }
    }
    Ok(graph)
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
        Some(Token::ParenL) => {}
        _ => return Err("Missing node inputs".into()),
    };

    let inputs = pairs(iter, identifier)?;

    match iter.next() {
        Some(Token::ParenR) => {}
        _ => return Err("Unclosed node inputs".into()),
    };

    match iter.next() {
        Some(Token::CurlyL) => {}
        _ => return Err("Missing node attributes".into()),
    };

    let attributes = pairs(iter, value)?;

    match iter.next() {
        Some(Token::CurlyR) => {}
        _ => return Err("Unclosed node attributes".into()),
    };

    Ok(Node::new(kind, name, inputs, attributes))
}

fn pairs<T>(iter: &mut Tokens, mapper: ValueMapper<T>) -> Result<Vec<Pair<T>>, String> {
    let mut pairs = Vec::new();
    while let Some(token) = iter.peek() {
        match token {
            Token::Identifier(_) => pairs.push(pair(iter, mapper)?),
            _ => break,
        }
    }
    Ok(pairs)
}

type ValueMapper<T> = fn(&mut Tokens) -> Result<T, String>;

fn pair<T>(iter: &mut Tokens, mapper: ValueMapper<T>) -> Result<Pair<T>, String> {
    let key = match iter.next() {
        Some(Token::Identifier(name)) => name.into(),
        _ => return Err("Missing pair key".into()),
    };

    match iter.next() {
        Some(Token::Colon) => {}
        _ => return Err("Missing pair separator".into()),
    };

    let value = mapper(iter)?;

    if let Some(Token::Comma) = iter.peek() {
        iter.next();
    }

    Ok(Pair::new(key, value))
}

fn literal(iter: &mut Tokens) -> Result<Literal, String> {
    match iter.peek() {
        Some(token) => Ok(match token {
            Token::Identifier(_) => Literal::Identifier(identifier(iter)?),
            _ => Literal::Value(value(iter)?),
        }),
        None => Err("Invalid value".into()),
    }
}

fn value(iter: &mut Tokens) -> Result<Value, String> {
    match iter.next() {
        Some(token) => Ok(match token {
            Token::Text(value) => Value::Text(value.into()),
            Token::Real(value) => Value::Real(*value),
            Token::Integer(value) => Value::Integer(*value),
            Token::True => Value::Boolean(true),
            Token::False => Value::Boolean(false),
            other => return Err(format!("Invalid value: {:?}", other)),
        }),
        None => Err("Invalid value".into()),
    }
}

fn identifier(iter: &mut Tokens) -> Result<String, String> {
    match iter.next() {
        Some(token) => Ok(match token {
            Token::Identifier(name) => name.into(),
            other => return Err(format!("Invalid value: {:?}", other)),
        }),
        None => Err("Invalid value".into()),
    }
}
