use std::fmt;
use std::rc::Rc;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Number(a), Object::Number(b)) => a.to_bits() == b.to_bits(), // Compare bitwise representation of `f64`
            (Object::String(a), Object::String(b)) => a == b,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::Nil, Object::Nil) => true,
            _ => false,
        }
    }
}

impl Eq for Object {}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Object::Number(value) => {
                // Convert the f64 to its bitwise representation for consistent hashing
                value.to_bits().hash(state);
            }
            Object::String(value) => {
                value.hash(state);
            }
            Object::Boolean(value) => {
                value.hash(state);
            }
            Object::Nil => {
                ().hash(state); // Hash `Nil` as an empty tuple
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Callable(Box<dyn crate::interpreter::lox_callable::LoxCallable>),
    Instance(Rc<crate::interpreter::lox_instance::LoxInstance>),
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