use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::interpreter::environment::{ Environment, Value};
use crate::interpreter::stmt::{Stmt, StmtVisitor};
use crate::interpreter::runtime_error::RuntimeError;
use crate::interpreter::token::Token;
use crate::interpreter::token_type::TokenType;
use crate::interpreter::expr::{Expr, ExprVisitor};
use crate::interpreter::lox_callable::LoxCallable;

pub struct Interpreter {
    globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
    locals: HashMap<Expr, usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));

        // Define a native function "clock"
        globals.borrow_mut().define(
            "clock",
            Value::Callable(LoxCallable::Native {
                arity: 0,
                call: Box::new(|_, _| {
                    let current_time = chrono::Utc::now().timestamp_millis() as f64 / 1000.0;
                    Value::Number(current_time)
                }),
            }),
        )

        Self {
            globals: globals.clone(),
            environment: globals,
            locals: HashMap::new(),
        }
    }

    /// Interprets a list of statements.
    pub fn interpret(&mut self, statements: &[Stmt]) {
        for stmt in statements {
            self.execute(stmt);
        }
    }

    /// Resolves an expression's depth for variable lookups.
    pub fn resolve(&mut self, expr: &Expr, depth: usize) {
        self.locals.insert(expr.clone(), depth);
    }

    /// Executes a block of statements in a new environment.
    pub fn execute_block(&mut self, statements: &[Stmt], new_env: Rc<RefCell<Environment>>) {
        let previous_env = self.environment.clone();
        self.environment = new_env;

        for stmt in statements {
            self.execute(stmt);
        }

        self.environment = previous_env; // Restore the previous environment
    }

    fn look_up_variable(&self, name: &Token, expr: &Expr) -> Value {
        if let Some(distance) = self.locals.get(expr) {
            self.environment.borrow().get_at(*distance, &name.lexeme)
        } else {
            self.globals.borrow().get(name)
        }
    }

    /// Evaluates an expression.
    fn evaluate(&mut self, expr: &Expr) -> Value {
        expr.accept(self)
    }

    /// Executes a statement.
    fn execute(&mut self, stmt: &Stmt) {
        stmt.accept(self);
    }
}

// Implement ExprVisitor for Interpreter
impl ExprVisitor<Value> for Interpreter {
    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Value {
        let evaluated_value = self.evaluate(value);

        if let Some(distance) = self.locals.get(&Expr::Variable(name.clone())) {
            self.environment
                .borrow_mut()
                .assign_at(*distance, name, evaluated_value.clone());
        } else {
            self.globals
                .borrow_mut()
                .assign(name, evaluated_value.clone());
        }

        evaluated_value
    }

    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Value {
        let left = self.evaluate(left);
        let right = self.evaluate(right);

        match operator.token_type {
            TokenType::Plus => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                (Value::String(a), Value::String(b)) => Value::String(a + &b),
                _ => panic!("{}", RuntimeError::new(
                    operator,
                    "Operands must be two numbers or two strings."
                )),
            },
            TokenType::Minus => {
                let (a, b) = Self::number_operands(operator, &left, &right);
                Value::Number(a - b)
            }
            TokenType::Slash => {
                let (a, b) = Self::number_operands(operator, &left, &right);
                Value::Number(a / b)
            }
            TokenType::Star => {
                let (a, b) = Self::number_operands(operator, &left, &right);
                Value::Number(a * b)
            }
            // Add other binary operations
            _ => panic!("Unsupported binary operator"),
        }
    }

    // Implement other `ExprVisitor` methods...
}

// Implement StmtVisitor for Interpreter
impl StmtVisitor<()> for Interpreter {
    fn visit_block_stmt(&mut self, statements: &[Stmt]) {
        let new_env = Rc::new(RefCell::new(Environment::with_enclosing(
            self.environment.clone(),
        )));
        self.execute_block(statements, new_env);
    }

    fn visit_expression_stmt(&mut self, expression: &Expr) {
        self.evaluate(expression);
    }

    fn visit_print_stmt(&mut self, expression: &Expr) {
        let value = self.evaluate(expression);
        println!("{}", value);
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) {
        let value = if let Some(expr) = initializer {
            self.evaluate(expr)
        } else {
            Value::Nil
        };
        self.environment.borrow_mut().define(name.lexeme.clone(), value);
    }

    // Implement other `StmtVisitor` methods...
}

// Utility functions
impl Interpreter {
    fn number_operands(operator: &Token, left: &Value, right: &Value) -> (f64, f64) {
        if let (Value::Number(a), Value::Number(b)) = (left, right) {
            (*a, *b)
        } else {
            panic!("{}", RuntimeError::new(
                operator,
                "Operands must be numbers."
            ));
        }
    }
}