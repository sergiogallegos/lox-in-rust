use crate::interpreter::stmt::{Stmt, StmtVisitor};
use crate::interpreter::token::Token;
use crate::interpreter::token_type::TokenType;
use crate::interpreter::expr::{Expr, ExprVisitor};

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter
    }

    pub fn print_expr(&self, expr: &Expr) -> String {
        expr.accept(self)
    }

    pub fn print_stmt(&self, stmt: &Stmt) -> String {
        stmt.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.accept(self));
        }
        builder.push(')');
        builder
    }

    fn parenthesize2(&self, name: &str, parts: &[&dyn ToString]) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);

        for part in parts {
            builder.push(' ');
            builder.push_str(&part.to_string());
        }

        builder.push(')');
        builder
    }
}

// Implement the ExprVisitor trait for AstPrinter
impl ExprVisitor<String> for AstPrinter {
    fn visit_assign_expr(&self, name: &Token, value: &Expr) -> String {
        self.parenthesize2("=", &[&name.lexeme, value])
    }

    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[left, right])
    }

    fn visit_call_expr(&self, callee: &Expr, arguments: &[Expr]) -> String {
        let mut parts: Vec<&dyn ToString> = vec![callee];
        parts.extend(arguments.iter().map(|arg| arg as &dyn ToString));
        self.parenthesize2("call", &parts)
    }

    fn visit_get_expr(&self, object: &Expr, name: &Token) -> String {
        self.parenthesize2(".", &[object, &name.lexeme])
    }

    fn visit_grouping_expr(&self, expression: &Expr) -> String {
        self.parenthesize("group", &[expression])
    }

    fn visit_literal_expr(&self, value: &Option<TokenType>) -> String {
        match value {
            Some(v) => format!("{:?}", v),
            None => "nil".to_string(),
        }
    }

    fn visit_logical_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[left, right])
    }

    fn visit_set_expr(&self, object: &Expr, name: &Token, value: &Expr) -> String {
        self.parenthesize2("=", &[object, &name.lexeme, value])
    }

    fn visit_super_expr(&self, method: &Token) -> String {
        self.parenthesize2("super", &[&method.lexeme])
    }

    fn visit_this_expr(&self) -> String {
        "this".to_string()
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[right])
    }

    fn visit_variable_expr(&self, name: &Token) -> String {
        name.lexeme.clone()
    }
}

// Implement the StmtVisitor trait for AstPrinter
impl StmtVisitor<String> for AstPrinter {
    fn visit_block_stmt(&self, statements: &[Stmt]) -> String {
        let mut builder = String::new();
        builder.push_str("(block ");
        for statement in statements {
            builder.push_str(&statement.accept(self));
        }
        builder.push(')');
        builder
    }

    fn visit_class_stmt(&self, name: &Token, superclass: &Option<Expr>, methods: &[Stmt]) -> String {
        let mut builder = String::new();
        builder.push_str("(class ");
        builder.push_str(&name.lexeme);
        if let Some(superclass) = superclass {
            builder.push_str(" < ");
            builder.push_str(&self.print_expr(superclass));
        }
        for method in methods {
            builder.push_str(" ");
            builder.push_str(&method.accept(self));
        }
        builder.push(')');
        builder
    }

    fn visit_expression_stmt(&self, expression: &Expr) -> String {
        self.parenthesize(";", &[expression])
    }

    fn visit_function_stmt(&self, name: &Token, params: &[Token], body: &[Stmt]) -> String {
        let mut builder = String::new();
        builder.push_str("(fun ");
        builder.push_str(&name.lexeme);
        builder.push('(');
        for (i, param) in params.iter().enumerate() {
            if i > 0 {
                builder.push(' ');
            }
            builder.push_str(&param.lexeme);
        }
        builder.push_str(") ");
        for stmt in body {
            builder.push_str(&stmt.accept(self));
        }
        builder.push(')');
        builder
    }

    fn visit_print_stmt(&self, expression: &Expr) -> String {
        self.parenthesize("print", &[expression])
    }

    fn visit_return_stmt(&self, value: &Option<Expr>) -> String {
        match value {
            Some(expr) => self.parenthesize("return", &[expr]),
            None => "(return)".to_string(),
        }
    }

    fn visit_var_stmt(&self, name: &Token, initializer: &Option<Expr>) -> String {
        match initializer {
            Some(expr) => self.parenthesize2("var", &[&name.lexeme, "=", expr]),
            None => self.parenthesize2("var", &[&name.lexeme]),
        }
    }

    fn visit_while_stmt(&self, condition: &Expr, body: &Stmt) -> String {
        self.parenthesize2("while", &[condition, body])
    }
}