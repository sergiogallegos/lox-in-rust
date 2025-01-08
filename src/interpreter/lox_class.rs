use std::collections::HashMap;
use std::rc::Rc;
use std::fmt;

use crate::interpreter::lox_callable::LoxCallable;
use crate::interpreter::lox_function::LoxFunction;
use crate::interpreter::lox_instance::LoxInstance;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value; // Assuming `Value` is defined elsewhere.

/// Represents a Lox class.
#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: String,
    pub superclass: Option<Rc<LoxClass>>,
    pub methods: HashMap<String, Rc<LoxFunction>>,
}

impl LoxClass {
    /// Creates a new LoxClass.
    pub fn new(
        name: String,
        superclass: Option<Rc<LoxClass>>,
        methods: HashMap<String, Rc<LoxFunction>>,
    ) -> Self {
        Self {
            name,
            superclass,
            methods,
        }
    }

    /// Finds a method by name in the class or its superclass.
    pub fn find_method(&self, name: &str) -> Option<Rc<LoxFunction>> {
        if let Some(method) = self.methods.get(name) {
            return Some(method.clone());
        }

        if let Some(superclass) = &self.superclass {
            return superclass.find_method(name);
        }

        None
    }
}

impl LoxCallable for LoxClass {
    /// Returns the number of arguments expected by the initializer (if it exists).
    fn arity(&self) -> usize {
        if let Some(initializer) = self.find_method("init") {
            initializer.arity()
        } else {
            0
        }
    }

    /// Calls the class, creating a new instance and initializing it if an initializer is defined.
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value {
        let instance = Rc::new(LoxInstance::new(Rc::new(self.clone())));

        if let Some(initializer) = self.find_method("init") {
            initializer.bind(instance.clone()).call(interpreter, arguments);
        }

        Value::Instance(instance)
    }

    /// Returns the string representation of the class.
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}