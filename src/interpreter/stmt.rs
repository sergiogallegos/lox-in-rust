use crate::interpreter::token::Token;
use crate::interpreter::expr::Expr;

pub trait StmtVisitor<R> {
    fn visit_block_stmt(&mut self, stmt: &StmtBlock) -> R;
    fn visit_class_stmt(&mut self, stmt: &StmtClass) -> R;
    fn visit_expression_stmt(&mut self, stmt: &StmtExpression) -> R;
    fn visit_function_stmt(&mut self, stmt: &StmtFunction) -> R;
    fn visit_if_stmt(&mut self, stmt: &StmtIf) -> R;
    fn visit_print_stmt(&mut self, stmt: &StmtPrint) -> R;
    fn visit_return_stmt(&mut self, stmt: &StmtReturn) -> R;
    fn visit_var_stmt(&mut self, stmt: &StmtVar) -> R;
    fn visit_while_stmt(&mut self, stmt: &StmtWhile) -> R;
}

#[derive(Debug)]
pub enum Stmt {
    Block(StmtBlock),
    Class(StmtClass),
    Expression(StmtExpression),
    Function(StmtFunction),
    If(StmtIf),
    Print(StmtPrint),
    Return(StmtReturn),
    Var(StmtVar),
    While(StmtWhile),
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &mut dyn StmtVisitor<R>) -> R {
        match self {
            Stmt::Block(stmt) => visitor.visit_block_stmt(stmt),
            Stmt::Class(stmt) => visitor.visit_class_stmt(stmt),
            Stmt::Expression(stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::Function(stmt) => visitor.visit_function_stmt(stmt),
            Stmt::If(stmt) => visitor.visit_if_stmt(stmt),
            Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
            Stmt::Return(stmt) => visitor.visit_return_stmt(stmt),
            Stmt::Var(stmt) => visitor.visit_var_stmt(stmt),
            Stmt::While(stmt) => visitor.visit_while_stmt(stmt),
        }
    }
}

impl StmtFunction {
    pub fn accept<R>(&self, visitor: &mut dyn StmtVisitor<R>) -> R {
        visitor.visit_function_stmt(self)
    }
}


// Nested Stmt structs

#[derive(Debug)]
pub struct StmtBlock {
    pub statements: Vec<Stmt>,
}

#[derive(Debug)]
pub struct StmtClass {
    pub name: Token,
    pub superclass: Option<Expr>,
    pub methods: Vec<StmtFunction>,
}

#[derive(Debug)]
pub struct StmtExpression {
    pub expression: Expr,
}

#[derive(Debug)]
pub struct StmtFunction {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub struct StmtIf {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Debug)]
pub struct StmtPrint {
    pub expression: Expr,
}

#[derive(Debug)]
pub struct StmtReturn {
    pub keyword: Token,
    pub value: Option<Expr>,
}

#[derive(Debug)]
pub struct StmtVar {
    pub name: Token,
    pub initializer: Option<Expr>,
}

#[derive(Debug)]
pub struct StmtWhile {
    pub condition: Expr,
    pub body: Box<Stmt>,
}