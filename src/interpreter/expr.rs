use crate::interpreter::token::Token;
use crate::interpreter::value::Object;

#[derive(Debug, Clone)]
pub enum Expr {
    Assign { name: Token, value: Box<Expr> },
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Call { callee: Box<Expr>, arguments: Vec<Expr> },
    Grouping { expression: Box<Expr> },
    Literal { value: Option<Object> },
    Logical { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Unary { operator: Token, right: Box<Expr> },
    Variable { name: Token },
}

pub trait ExprVisitor<R> {
    fn visit_assign_expr(&mut self, expr: &Expr) -> R;
    fn visit_binary_expr(&mut self, expr: &Expr) -> R;
    fn visit_call_expr(&mut self, expr: &Expr) -> R;
    fn visit_grouping_expr(&mut self, expr: &Expr) -> R;
    fn visit_literal_expr(&mut self, expr: &Expr) -> R;
    fn visit_logical_expr(&mut self, expr: &Expr) -> R;
    fn visit_unary_expr(&mut self, expr: &Expr) -> R;
    fn visit_variable_expr(&mut self, expr: &Expr) -> R;
    fn visit_get_expr(&mut self, expr: &Expr) -> R;
    fn visit_set_expr(&mut self, expr: &Expr) -> R;
    fn visit_super_expr(&mut self, expr: &Expr) -> R;
    fn visit_this_expr(&mut self, expr: &Expr) -> R;
}
