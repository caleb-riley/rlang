use crate::value::Operator;

pub enum Decl {
    FnDecl(FnDecl),
}

pub struct FnDecl {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

pub enum Stmt {
    FnCall(FnCall),
    Return(ReturnStmt),
    If(IfStmt),
    While(WhileStmt),
    Assign(AssignStmt),
    Decl(DeclStmt),
}

pub struct AssignStmt {
    pub var: String,
    pub val: Expr,
}

pub struct DeclStmt {
    pub var: String,
    pub val: Expr,
}

pub struct ReturnStmt {
    pub expr: Expr,
}

pub struct IfStmt {
    pub cond: Expr,
    pub body: Vec<Stmt>,
}

pub struct WhileStmt {
    pub cond: Expr,
    pub body: Vec<Stmt>,
}

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

pub struct FieldAccess {
    pub obj: Box<Expr>,
    pub field: String,
}

pub struct Binary {
    pub op: Operator,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

pub struct Unary {
    pub op: Operator,
    pub expr: Box<Expr>,
}

pub struct FnCall {
    pub name: String,
    pub args: Vec<Expr>,
}
