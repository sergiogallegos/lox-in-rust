use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::environment::Environment;
use crate::interpreter::lox_instance::LoxInstance;
use crate::interpreter::stmt::Stmt;

/// Represents a function in Lox.
#[derive(Clone)]
pub struct LoxFunction {
    declaration: Rc<Stmt>,           // Function declaration
    closure: Rc<RefCell<Environment>>, // Closure environment
    is_initializer: bool,           // Indicates if this is an initializer
}

impl LoxFunction {
    /// Creates a new LoxFunction.
    pub fn new(declaration: Rc<Stmt>, closure: Rc<RefCell<Environment>>, is_initializer: bool) -> Self {
        Self {
            declaration,
            closure,
            is_initializer,
        }
    }

    /// Binds the function to an instance, defining `this` in the environment.
    pub fn bind(&self, instance: Rc<LoxInstance>) -> Self {
        let environment = Rc::new(RefCell::new(Environment::with_enclosing(self.closure.clone())));
        environment.borrow_mut().define("this", Value::Instance(instance));
        Self::new(self.declaration.clone(), environment, self.is_initializer)
    }
}

impl crate::interpreter::lox_callable::LoxCallable for LoxFunction {
    /// Returns the number of parameters the function expects.
    fn arity(&self) -> usize {
        if let Stmt::Function { params, .. } = &*self.declaration {
            params.len()
        } else {
            0
        }
    }

    /// Executes the function in its closure environment.
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value {
        let environment = Rc::new(RefCell::new(Environment::with_enclosing(self.closure.clone())));

        if let Stmt::Function { params, body, .. } = &*self.declaration {
            for (i, param) in params.iter().enumerate() {
                if let Some(argument) = arguments.get(i) {
                    environment.borrow_mut().define(&param.lexeme, argument.clone());
                }
            }

            // Execute the function body
            if let Err(return_value) = interpreter.execute_block(body, environment) {
                // Handle return statement or initializer
                if self.is_initializer {
                    return self.closure.borrow().get_at(0, "this").unwrap();
                }
                return return_value;
            }
        }

        // Return the instance if this is an initializer, or nil otherwise
        if self.is_initializer {
            self.closure.borrow().get_at(0, "this").unwrap()
        } else {
            Value::Nil
        }
    }

    /// String representation of the function.
    fn to_string(&self) -> String {
        if let Stmt::Function { name, .. } = &*self.declaration {
            format!("<fn {}>", name.lexeme)
        } else {
            "<fn unknown>".to_string()
        }
    }
}

impl fmt::Debug for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}