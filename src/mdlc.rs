use crate::utils::Value;
use std::{collections::HashMap, iter::Peekable, slice::Iter, str::Chars};

type TokenMap = HashMap<String, Token>;
type Tokens<'a> = Peekable<Iter<'a, Token>>;

#[derive(Debug, Clone)]
pub enum Statement {
    Assign {
        member: Member,
        value: Value,
    },
    New {
        kind: String,
        name: String,
    },
    Delete {
        name: String,
    },
    Glob {
        attr: String,
        value: Literal,
    },
    Wire {
        downstream: Member,
        upstream: String,
    },
}

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    Value(Value),
}

#[derive(Debug, Clone)]
pub struct Member {
    node: String,
    attr: String,
}

#[derive(Debug, Clone)]
enum Token {
    Identifier(String),
    Value(Value),

    Equal,
    Dot,
    Arrow,

    Glob,
    New,
    Delete,
}

#[derive(Debug, Clone)]
pub struct Parser {
    keywords: TokenMap,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new() -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("glob".into(), Token::Glob);
        keywords.insert("new".into(), Token::New);
        keywords.insert("delete".into(), Token::Delete);
        keywords.insert("true".into(), Token::Value(Value::Boolean(true)));
        keywords.insert("false".into(), Token::Value(Value::Boolean(false)));
        Self {
            keywords,
            tokens: Vec::new(),
        }
    }

    pub fn parse(&self, src: &str) -> Result<Statement, String> {
        let tokens = tokenize(src, &self.keywords)?;
        parse(&tokens)
    }
}

fn tokenize(src: &str, keywords: &TokenMap) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::with_capacity(16);
    let mut iter = src.chars().peekable();
    while let Some(c) = iter.next() {
        tokens.push(match c {
            '=' => Ok(Token::Equal),
            '.' => Ok(Token::Dot),
            '<' => match iter.next() {
                Some('-') => Ok(Token::Arrow),
                _ => Err("Unrecognized token".into()),
            },
            '#' => break,
            '"' => {
                let text = consume(&mut iter, |c| c != '"', None);
                iter.next();
                Ok(Token::Value(Value::Text(text)))
            }
            c => {
                if c.is_ascii_alphabetic() {
                    let text = consume(&mut iter, |c| c.is_ascii_alphabetic() || c == '_', Some(c));
                    match keywords.get(&text) {
                        Some(token) => Ok(token.clone()),
                        None => Ok(Token::Identifier(text)),
                    }
                } else if c.is_ascii_digit() || c == '-' {
                    let text = consume(&mut iter, |c| c.is_ascii_digit() || c == '.', Some(c));
                    match text.parse::<isize>() {
                        Ok(value) => Ok(Token::Value(Value::Integer(value))),
                        Err(_) => match text.parse::<f32>() {
                            Ok(value) => Ok(Token::Value(Value::Real(value))),
                            Err(_) => Err("Could not parse number".into()),
                        },
                    }
                } else if c.is_ascii_whitespace() {
                    continue;
                } else {
                    Err("Unrecognized token".to_string())
                }
            }
        }?);
    }
    Ok(tokens)
}

fn consume(iter: &mut Peekable<Chars>, matcher: fn(char) -> bool, seed: Option<char>) -> String {
    let mut text = String::new();
    if let Some(seed) = seed {
        text.push(seed);
    }
    while let Some(c) = iter.peek() {
        if matcher(*c) {
            text.push(iter.next().unwrap())
        } else {
            break;
        }
    }
    text
}

// TODO: Better help for incorrect statements
fn parse(tokens: &[Token]) -> Result<Statement, String> {
    let mut iter = tokens.iter().peekable();
    match iter.peek() {
        Some(token) => match token {
            Token::Glob => glob(&mut iter).map_err(|_| "Invalid glob".into()),
            Token::New => new(&mut iter).map_err(|_| "Invalid new".into()),
            Token::Delete => delete(&mut iter).map_err(|_| "Invalid delete".into()),
            Token::Identifier(_) => set(&mut iter),
            _ => Err("Unrecognized statement".into()),
        },
        None => Err("Empty statement".into()),
    }
}

fn glob(iter: &mut Tokens) -> Result<Statement, ()> {
    iter.next();
    let attr = ident(iter)?;
    equal(iter)?;
    let value = literal(iter)?;
    Ok(Statement::Glob { attr, value })
}

fn new(iter: &mut Tokens) -> Result<Statement, ()> {
    iter.next();
    let kind = ident(iter)?;
    let name = ident(iter)?;
    Ok(Statement::New { kind, name })
}

fn delete(iter: &mut Tokens) -> Result<Statement, ()> {
    iter.next();
    let name = ident(iter)?;
    Ok(Statement::Delete { name })
}

fn set(iter: &mut Tokens) -> Result<Statement, String> {
    let member = member(iter).map_err(|_| "Unrecognized member".to_string())?;
    match iter.next() {
        Some(Token::Arrow) => wire(iter, member).map_err(|_| "Invalid wire".into()),
        Some(Token::Equal) => assign(iter, member).map_err(|_| "Invalid assignment".into()),
        _ => Err("Unrecognized member statement".into()),
    }
}

fn assign(iter: &mut Tokens, member: Member) -> Result<Statement, ()> {
    let value = value(iter)?;
    Ok(Statement::Assign { member, value })
}

fn wire(iter: &mut Tokens, downstream: Member) -> Result<Statement, ()> {
    let upstream = ident(iter)?;
    Ok(Statement::Wire {
        upstream,
        downstream,
    })
}

fn literal(iter: &mut Tokens) -> Result<Literal, ()> {
    match iter.next() {
        Some(Token::Value(value)) => Ok(Literal::Value(value.clone())),
        Some(Token::Identifier(name)) => Ok(Literal::Identifier(name.clone())),
        _ => Err(()),
    }
}

fn value(iter: &mut Tokens) -> Result<Value, ()> {
    match iter.next() {
        Some(Token::Value(value)) => Ok(value.clone()),
        _ => Err(()),
    }
}

fn member(iter: &mut Tokens) -> Result<Member, ()> {
    let node = ident(iter)?;
    dot(iter)?;
    let attr = ident(iter)?;
    Ok(Member { node, attr })
}

fn dot(iter: &mut Tokens) -> Result<(), ()> {
    match iter.next() {
        Some(Token::Dot) => Ok(()),
        _ => Err(()),
    }
}

fn equal(iter: &mut Tokens) -> Result<(), ()> {
    match iter.next() {
        Some(Token::Equal) => Ok(()),
        _ => Err(()),
    }
}

fn ident(iter: &mut Tokens) -> Result<String, ()> {
    match iter.next() {
        Some(Token::Identifier(name)) => Ok(name.clone()),
        _ => Err(()),
    }
}
