use crate::interpreter::expr::{Expr, ExprVisitor};
use crate::interpreter::stmt::{
    Stmt, StmtBlock, StmtClass, StmtExpression, StmtFunction, StmtIf,
    StmtPrint, StmtReturn, StmtVar, StmtWhile, StmtVisitor,
};
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

    fn visit_get_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Get { object, name } = expr {
            self.parenthesize2(".", &[object, &name.lexeme])
        } else {
            String::new()
        }
    }

    fn visit_set_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Set { object, name, value } = expr {
            self.parenthesize2("=", &[object, &name.lexeme, value])
        } else {
            String::new()
        }
    }

    fn visit_super_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Super { method } = expr {
            self.parenthesize2("super", &[&method.lexeme])
        } else {
            String::new()
        }
    }

    fn visit_this_expr(&mut self, expr: &Expr) -> String {
        "this".to_string()
    }
}

impl StmtVisitor<String> for AstPrinter {
    fn visit_block_stmt(&mut self, stmt: &StmtBlock) -> String {
        let mut builder = String::new();
        builder.push_str("(block ");
        for statement in &stmt.statements {
            builder.push_str(&statement.accept(self));
        }
        builder.push(')');
        builder
    }

    fn visit_expression_stmt(&mut self, stmt: &StmtExpression) -> String {
        self.parenthesize(";", &[&stmt.expression])
    }

    fn visit_print_stmt(&mut self, stmt: &StmtPrint) -> String {
        self.parenthesize("print", &[&stmt.expression])
    }

    fn visit_class_stmt(&mut self, stmt: &StmtClass) -> String {
        let mut builder = String::new();
        builder.push_str("(class ");
        builder.push_str(&stmt.name.lexeme);
        if let Some(superclass) = &stmt.superclass {
            builder.push_str(" < ");
            builder.push_str(&superclass.to_string());
        }
        for method in &stmt.methods {
            builder.push_str(" ");
            builder.push_str(&method.accept(self));
        }
        builder.push(')');
        builder
    }

    fn visit_function_stmt(&mut self, stmt: &StmtFunction) -> String {
        let mut builder = String::new();
        builder.push_str("(fun ");
        builder.push_str(&stmt.name.lexeme);
        builder.push('(');
        for (i, param) in stmt.params.iter().enumerate() {
            if i > 0 {
                builder.push(' ');
            }
            builder.push_str(&param.lexeme);
        }
        builder.push_str(") ");
        for body_stmt in &stmt.body {
            builder.push_str(&body_stmt.accept(self));
        }
        builder.push(')');
        builder
    }

    fn visit_if_stmt(&mut self, stmt: &StmtIf) -> String {
        let mut builder = String::new();
        builder.push_str("(if ");
        builder.push_str(&stmt.condition.to_string());
        builder.push(' ');
        builder.push_str(&stmt.then_branch.accept(self));
        if let Some(else_branch) = &stmt.else_branch {
            builder.push_str(" else ");
            builder.push_str(&else_branch.accept(self));
        }
        builder.push(')');
        builder
    }

    fn visit_return_stmt(&mut self, stmt: &StmtReturn) -> String {
        let mut builder = String::new();
        builder.push_str("(return");
        if let Some(value) = &stmt.value {
            builder.push(' ');
            builder.push_str(&value.to_string());
        }
        builder.push(')');
        builder
    }

    fn visit_var_stmt(&mut self, stmt: &StmtVar) -> String {
        let mut builder = String::new();
        builder.push_str("(var ");
        builder.push_str(&stmt.name.lexeme);
        if let Some(initializer) = &stmt.initializer {
            builder.push_str(" = ");
            builder.push_str(&initializer.to_string());
        }
        builder.push(')');
        builder
    }

    fn visit_while_stmt(&mut self, stmt: &StmtWhile) -> String {
        let mut builder = String::new();
        builder.push_str("(while ");
        builder.push_str(&stmt.condition.to_string());
        builder.push(' ');
        builder.push_str(&stmt.body.accept(self));
        builder.push(')');
        builder
    }
}
