use std::collections::{HashMap, VecDeque};
use crate::interpreter::{expr::Expr, stmt::Stmt, token::Token};
use crate::interpreter::{lox::Lox, interpreter::Interpreter};
use crate::interpreter::expr::ExprVisitor;
use crate::interpreter::stmt::StmtVisitor;

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: VecDeque<HashMap<String, bool>>,
    current_function: FunctionType,
    current_class: ClassType,
}

#[derive(Debug, Clone, Copy)]
enum FunctionType {
    None,
    Function,
    Initializer,
    Method,
}

#[derive(Debug, Clone, Copy)]
enum ClassType {
    None,
    Class,
    Subclass,
}

impl<'a> Resolver<'a> {
    /// Creates a new Resolver.
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: VecDeque::new(),
            current_function: FunctionType::None,
            current_class: ClassType::None,
        }
    }

    /// Resolves a list of statements.
    pub fn resolve(&mut self, statements: &[Stmt]) {
        for statement in statements {
            self.resolve_stmt(statement);
        }
    }

    /// Resolves a single statement.
    fn resolve_stmt(&mut self, stmt: &Stmt) {
        stmt.accept(self);
    }

    /// Resolves a single expression.
    fn resolve_expr(&mut self, expr: &Expr) {
        expr.accept(self);
    }

    /// Resolves a function, including its parameters and body.
    fn resolve_function(&mut self, function: &Stmt, func_type: FunctionType) {
        let enclosing_function = self.current_function;
        self.current_function = func_type;

        self.begin_scope();
        if let Stmt::Function { params, body, .. } = function {
            for param in params {
                self.declare(param);
                self.define(param);
            }
            self.resolve(body);
        }
        self.end_scope();
        self.current_function = enclosing_function;
    }

    /// Begins a new scope.
    fn begin_scope(&mut self) {
        self.scopes.push_back(HashMap::new());
    }

    /// Ends the current scope.
    fn end_scope(&mut self) {
        self.scopes.pop_back();
    }

    /// Declares a variable in the current scope.
    fn declare(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.back_mut() {
            if scope.contains_key(&name.lexeme) {
                Lox::error(name, "Variable already declared in this scope.");
            }
            scope.insert(name.lexeme.clone(), false);
        }
    }

    /// Defines a variable in the current scope.
    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.back_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }

    /// Resolves a local variable by checking its scope.
    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, i);
                return;
            }
        }
    }
}

impl<'a> ExprVisitor for Resolver<'a> {
    type Output = ();

    fn visit_variable_expr(&mut self, expr: &Expr::Variable) {
        if let Some(scope) = self.scopes.back() {
            if let Some(false) = scope.get(&expr.name.lexeme) {
                Lox::error(&expr.name, "Cannot read variable in its own initializer.");
            }
        }
        self.resolve_local(expr, &expr.name);
    }

    fn visit_assign_expr(&mut self, expr: &Expr::Assign) {
        self.resolve_expr(&expr.value);
        self.resolve_local(expr, &expr.name);
    }

    fn visit_binary_expr(&mut self, expr: &Expr::Binary) {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
    }

    fn visit_call_expr(&mut self, expr: &Expr::Call) {
        self.resolve_expr(&expr.callee);
        for argument in &expr.arguments {
            self.resolve_expr(argument);
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr::Grouping) {
        self.resolve_expr(&expr.expression);
    }

    fn visit_literal_expr(&mut self, _expr: &Expr::Literal) {}

    fn visit_unary_expr(&mut self, expr: &Expr::Unary) {
        self.resolve_expr(&expr.right);
    }
}

impl<'a> StmtVisitor for Resolver<'a> {
    type Output = ();

    fn visit_block_stmt(&mut self, stmt: &Stmt::Block) {
        self.begin_scope();
        self.resolve(&stmt.statements);
        self.end_scope();
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt::Var) {
        self.declare(&stmt.name);
        if let Some(initializer) = &stmt.initializer {
            self.resolve_expr(initializer);
        }
        self.define(&stmt.name);
    }

    fn visit_function_stmt(&mut self, stmt: &Stmt::Function) {
        self.declare(&stmt.name);
        self.define(&stmt.name);
        self.resolve_function(stmt, FunctionType::Function);
    }

    fn visit_expression_stmt(&mut self, stmt: &Stmt::Expression) {
        self.resolve_expr(&stmt.expression);
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt::If) {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.then_branch);
        if let Some(else_branch) = &stmt.else_branch {
            self.resolve_stmt(else_branch);
        }
    }

    fn visit_while_stmt(&mut self, stmt: &Stmt::While) {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.body);
    }

    fn visit_return_stmt(&mut self, stmt: &Stmt::Return) {
        if self.current_function == FunctionType::None {
            Lox::error(&stmt.keyword, "Cannot return from top-level code.");
        }
        if let Some(value) = &stmt.value {
            self.resolve_expr(value);
        }
    }

    fn visit_class_stmt(&mut self, stmt: &Stmt::Class) {
        let enclosing_class = self.current_class;
        self.current_class = ClassType::Class;

        self.declare(&stmt.name);
        self.define(&stmt.name);

        if let Some(superclass) = &stmt.superclass {
            if let Expr::Variable { name } = superclass {
                if stmt.name.lexeme == name.lexeme {
                    Lox::error(name, "A class cannot inherit from itself.");
                }
            }
            self.current_class = ClassType::Subclass;
            self.resolve_expr(superclass);
        }

        self.begin_scope();
        if let Some(scope) = self.scopes.back_mut() {
            scope.insert("this".to_string(), true);
        }

        for method in &stmt.methods {
            let declaration = if method.name.lexeme == "init" {
                FunctionType::Initializer
            } else {
                FunctionType::Method
            };
            self.resolve_function(method, declaration);
        }

        self.end_scope();
        self.current_class = enclosing_class;
    }
}