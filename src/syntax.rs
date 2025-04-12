use crate::value::Operator;

#[derive(Clone)]
pub enum Decl {
    FnDecl(FnDecl),
}

#[derive(Clone)]
pub struct FnDecl {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Clone)]
pub enum Stmt {
    FnCall(FnCall),
    Return(ReturnStmt),
    If(IfStmt),
    Assign(AssignStmt),
    Decl(DeclStmt),
}

#[derive(Clone)]
pub struct AssignStmt {
    pub var: String,
    pub val: Expr,
}

#[derive(Clone)]
pub struct DeclStmt {
    pub var: String,
    pub val: Expr,
}

#[derive(Clone)]
pub struct ReturnStmt {
    pub expr: Expr,
}

#[derive(Clone)]
pub struct IfStmt {
    pub cond: Expr,
    pub body: Vec<Stmt>,
}

#[derive(Clone)]
pub enum Expr {
    Identfier(String),
    NumberLiteral(i32),
    BooleanLiteral(bool),
    StringLiteral(String),
    NullLiteral,
    FnCall(FnCall),
    FieldAccess(FieldAccess),
    ObjectLiteral(Vec<(String, Expr)>),
    Binary(Binary),
    Unary(Unary),
}

#[derive(Clone)]
pub struct FieldAccess {
    pub obj: Box<Expr>,
    pub field: String,
}

#[derive(Clone)]
pub struct Binary {
    pub op: Operator,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Clone)]
pub struct Unary {
    pub op: Operator,
    pub expr: Box<Expr>,
}

#[derive(Clone)]
pub struct FnCall {
    pub name: String,
    pub args: Vec<Expr>,
}
