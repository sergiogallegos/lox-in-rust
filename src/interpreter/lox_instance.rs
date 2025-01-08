use std::collections::HashMap;
use std::rc::Rc;

use crate::interpreter::lox_class::LoxClass;
use crate::interpreter::runtime_error::RuntimeError;
use crate::interpreter::token::Token;
use crate::interpreter::value::Value;

/// Represents an instance of a Lox class.
#[derive(Debug, Clone)]
pub struct LoxInstance {
    klass: Rc<LoxClass>,           // The class this instance belongs to
    fields: HashMap<String, Value>, // Instance fields
}

impl LoxInstance {
    /// Creates a new instance of a class.
    pub fn new(klass: Rc<LoxClass>) -> Self {
        Self {
            klass,
            fields: HashMap::new(),
        }
    }

    /// Gets a property or method from the instance.
    pub fn get(&self, name: &Token) -> Value {
        // Check for instance fields first
        if let Some(value) = self.fields.get(&name.lexeme) {
            return value.clone();
        }

        // Check for methods in the class
        if let Some(method) = self.klass.find_method(&name.lexeme) {
            return Value::Callable(Box::new(method.bind(Rc::new(self.clone()))));
        }

        // If not found, throw a runtime error
        panic!(
            "{}",
            RuntimeError::new(name.clone(), format!("Undefined property '{}'.", name.lexeme))
        );
    }

    /// Sets a property on the instance.
    pub fn set(&mut self, name: &Token, value: Value) {
        self.fields.insert(name.lexeme.clone(), value);
    }
}

impl std::fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.klass.name)
    }
}