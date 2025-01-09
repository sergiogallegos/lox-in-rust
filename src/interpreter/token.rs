use crate::interpreter::token_type::TokenType;
use std::fmt;
use std::cmp::PartialEq;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize, // [location]
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Eq for Literal {}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::Number(a), Literal::Number(b)) => a == b,
            (Literal::String(a), Literal::String(b)) => a == b,
            (Literal::Boolean(a), Literal::Boolean(b)) => a == b,
            (Literal::Nil, Literal::Nil) => true,
            _ => false,
        }
    }
}

impl Hash for Literal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Literal::Number(value) => value.to_bits().hash(state),
            Literal::String(value) => value.hash(state),
            Literal::Boolean(value) => value.hash(state),
            Literal::Nil => ().hash(state),
        }
    }
}


impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        let literal_string = match &self.literal {
            Some(Literal::Number(n)) => n.to_string(),
            Some(Literal::String(s)) => s.clone(),
            Some(Literal::Boolean(b)) => b.to_string(),
            Some(Literal::Nil) | None => "nil".to_string(),
        };

        format!("{:?} {} {}", self.token_type, self.lexeme, literal_string)
    }
}

impl From<serde_json::Value> for Literal {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::String(s) => Literal::String(s),
            serde_json::Value::Number(n) => Literal::Number(n.as_f64().unwrap()), // Example
            serde_json::Value::Bool(b) => Literal::Boolean(b),
            _ => Literal::Nil, // Handle other cases
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({:?})", self.lexeme, self.token_type)
    }
}