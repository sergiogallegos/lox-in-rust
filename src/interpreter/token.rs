use crate::interpreter::token_type::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize, // [location]
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
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
