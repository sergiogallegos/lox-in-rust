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