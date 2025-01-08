use crate::interpreter::token::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl RuntimeError {
    /// Creates a new RuntimeError with the associated token and message.
    pub fn new(token: Token, message: String) -> Self {
        Self { token, message }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Runtime error at '{}': {}", self.token.lexeme, self.message)
    }
}

impl std::error::Error for RuntimeError {}
