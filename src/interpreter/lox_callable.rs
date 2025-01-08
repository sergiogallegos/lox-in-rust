use crate::interpreter::value::Value;
use crate::interpreter::interpreter::Interpreter;
use std::fmt;

/// Represents a callable function or class in the Lox language.
pub trait LoxCallable: dyn_clone::DynClone + std::fmt::Debug {
    /// Returns the number of arguments the callable expects.
    fn arity(&self) -> usize;

    /// Calls the function or class.
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value;

    /// Provides a string representation of the callable.
    fn to_string(&self) -> String {
        "<native fn>".to_string()
    }
}

dyn_clone::clone_trait_object!(LoxCallable);

/// A concrete implementation of LoxCallable for native functions.
#[derive(Debug, Clone)]
pub struct NativeFunction {
    pub arity: usize,
    pub function: fn(&mut Interpreter, Vec<Value>) -> Value,
}

impl LoxCallable for NativeFunction {
    fn arity(&self) -> usize {
        self.arity
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value {
        (self.function)(interpreter, arguments)
    }

    fn to_string(&self) -> String {
        "<native fn>".to_string()
    }
}

/// Helper implementation for debug string representation of `LoxCallable`.
impl fmt::Display for dyn LoxCallable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}