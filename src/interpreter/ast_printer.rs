use crate::interpreter::expr::{Expr, ExprVisitor};
use crate::interpreter::stmt::{Stmt, StmtVisitor};

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter
    }

    pub fn print_expr(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    pub fn print_stmt(&mut self, stmt: &Stmt) -> String {
        stmt.accept(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
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

    fn parenthesize2(&mut self, name: &str, parts: &[&dyn ToString]) -> String {
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
    fn visit_assign_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Assign { name, value } = expr {
            self.parenthesize2("=", &[&name.lexeme, value])
        } else {
            String::new()
        }
    }

    fn visit_binary_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Binary { left, operator, right } = expr {
            self.parenthesize(&operator.lexeme, &[left, right])
        } else {
            String::new()
        }
    }

    fn visit_call_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Call { callee, arguments } = expr {
            let mut parts: Vec<&dyn ToString> = vec![callee];
            parts.extend(arguments.iter().map(|arg| arg as &dyn ToString));
            self.parenthesize2("call", &parts)
        } else {
            String::new()
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Grouping { expression } = expr {
            self.parenthesize("group", &[expression])
        } else {
            String::new()
        }
    }

    fn visit_literal_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Literal { value } = expr {
            match value {
                Some(v) => format!("{:?}", v),
                None => "nil".to_string(),
            }
        } else {
            String::new()
        }
    }

    fn visit_logical_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Logical { left, operator, right } = expr {
            self.parenthesize(&operator.lexeme, &[left, right])
        } else {
            String::new()
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Unary { operator, right } = expr {
            self.parenthesize(&operator.lexeme, &[right])
        } else {
            String::new()
        }
    }

    fn visit_variable_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Variable { name } = expr {
            name.lexeme.clone()
        } else {
            String::new()
        }
    }
}

// Implement the StmtVisitor trait for AstPrinter
impl StmtVisitor<String> for AstPrinter {
    fn visit_block_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::Block { statements } = stmt {
            let mut builder = String::new();
            builder.push_str("(block ");
            for statement in statements {
                builder.push_str(&statement.accept(self));
            }
            builder.push(')');
            builder
        } else {
            String::new()
        }
    }

    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::Expression { expression } = stmt {
            self.parenthesize(";", &[expression])
        } else {
            String::new()
        }
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::Print { expression } = stmt {
            self.parenthesize("print", &[expression])
        } else {
            String::new()
        }
    }
}