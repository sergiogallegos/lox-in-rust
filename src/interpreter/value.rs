use std::fmt;

#[derive(Debug, Clone, PartialEq)]

pub enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Callable(Box<dyn crate::interpreter::lox_callable::LoxCallable>),
    Instance(crate::interpreter::lox_instance::LoxInstance),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Callable(_) => write!(f, "<callable>"),
            Value::Instance(instance) => write!(f, "{}", instance),
        }
    }
}