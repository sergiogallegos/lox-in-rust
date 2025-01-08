use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::interpreter::token::Token;
use crate::interpreter::runtime_error::RuntimeError;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Rc<RefCell<Environment>>>, // Reference to the enclosing environment
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Number(f64),
    String(String),
    Bool(bool),
    Callable, // You can extend this for Lox functions/objects.
    // Add other types as needed
}

impl Environment {
    /// Creates a new global environment.
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    /// Creates a new environment with an enclosing parent.
    pub fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    /// Gets the value of a variable by its name.
    pub fn get(&self, name: &Token) -> Value {
        if let Some(value) = self.values.get(&name.lexeme) {
            return value.clone();
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
        }

        panic!("{}", RuntimeError::new(name.clone(), format!("Undefined variable '{}'.", name.lexeme)));
    }

    /// Assigns a value to an existing variable.
    pub fn assign(&mut self, name: &Token, value: Value) {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return;
        }

        if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value);
            return;
        }

        panic!("{}", RuntimeError::new(name.clone(), format!("Undefined variable '{}'.", name.lexeme)));
    }

    /// Defines a new variable in the current environment.
    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value);
    }

    /// Gets an ancestor environment at a specific distance.
    pub fn ancestor(&self, distance: usize) -> Rc<RefCell<Environment>> {
        let mut environment = Rc::new(RefCell::new(self.clone()));

        for _ in 0..distance {
            let enclosing = environment
                .borrow()
                .enclosing
                .as_ref()
                .expect("Enclosing environment not found")
                .clone();
            environment = enclosing;
        }

        environment
    }

    /// Gets a variable value at a specific distance.
    pub fn get_at(&self, distance: usize, name: &str) -> Value {
        self.ancestor(distance).borrow().values.get(name).cloned().unwrap_or(Value::Nil)
    }

    /// Assigns a value to a variable at a specific distance.
    pub fn assign_at(&self, distance: usize, name: &Token, value: Value) {
        self.ancestor(distance)
            .borrow_mut()
            .values
            .insert(name.lexeme.clone(), value);
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = format!("{:?}", self.values);
        if let Some(enclosing) = &self.enclosing {
            result.push_str(" -> ");
            result.push_str(&format!("{}", enclosing.borrow()));
        }
        write!(f, "{}", result)
    }
}
