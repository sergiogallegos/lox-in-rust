use std::fmt;
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
    Get { object: Box<Expr>, name: Token },
    Set { object: Box<Expr>, name: Token, value: Box<Expr> },
    Super { method: Token },
    This,
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

impl Expr {
    pub fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
        match self {
            Expr::Assign { .. } => visitor.visit_assign_expr(self),
            Expr::Binary { .. } => visitor.visit_binary_expr(self),
            Expr::Call { .. } => visitor.visit_call_expr(self),
            Expr::Grouping { .. } => visitor.visit_grouping_expr(self),
            Expr::Literal { .. } => visitor.visit_literal_expr(self),
            Expr::Logical { .. } => visitor.visit_logical_expr(self),
            Expr::Unary { .. } => visitor.visit_unary_expr(self),
            Expr::Variable { .. } => visitor.visit_variable_expr(self),
            Expr::Get { .. } => visitor.visit_get_expr(self),
            Expr::Set { .. } => visitor.visit_set_expr(self),
            Expr::Super { .. } => visitor.visit_super_expr(self),
            Expr::This => visitor.visit_this_expr(self),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Assign { name, value } => write!(f, "Assign({}, {})", name.lexeme, value),
            Expr::Binary { left, operator, right } => write!(f, "Binary({}, {}, {})", left, operator, right),
            Expr::Call { callee, arguments } => write!(f, "Call({}, {:?})", callee, arguments),
            Expr::Grouping { expression } => write!(f, "Grouping({})", expression),
            Expr::Literal { value } => write!(f, "Literal({:?})", value),
            Expr::Logical { left, operator, right } => write!(f, "Logical({}, {}, {})", left, operator, right),
            Expr::Unary { operator, right } => write!(f, "Unary({}, {})", operator, right),
            Expr::Variable { name } => write!(f, "Variable({})", name.lexeme),
            Expr::Get { object, name } => write!(f, "Get({}, {})", object, name.lexeme),
            Expr::Set { object, name, value } => write!(f, "Set({}, {}, {})", object, name.lexeme, value),
            Expr::Super { method } => write!(f, "Super({})", method.lexeme),
            Expr::This => write!(f, "This"),
        }
    }
}