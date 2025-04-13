use crate::syntax::*;

fn display(text: impl Into<String>, indent: usize) {
    let padding = vec!["  "; indent].into_iter().collect::<String>();

    println!("{}{}", padding, text.into());
}

pub trait TreePrint {
    fn print(&self, indent: usize);
}

impl TreePrint for Stmt {
    fn print(&self, indent: usize) {
        match self {
            Stmt::FnCall(fn_call) => {
                display("FnCall", indent);
                display(" name:", indent);
                display(&fn_call.name, indent + 1);
                display(" args:", indent);

                for arg in fn_call.args.iter() {
                    arg.print(indent + 1);
                }
            }
            Stmt::Return(ret_stmt) => {
                display("ReturnStmt", indent);
                display(" expr:", indent);
                ret_stmt.expr.print(indent + 1);
            }
            Stmt::If(if_stmt) => {
                display("IfStmt", indent);
                display(" cond:", indent);
                if_stmt.cond.print(indent + 1);
                display(" body:", indent);

                for stmt in if_stmt.body.iter() {
                    stmt.print(indent + 1);
                }
            }
            Stmt::While(while_stmt) => {
                display("WhileStmt", indent);
                display(" cond:", indent);
                while_stmt.cond.print(indent + 1);
                display(" body:", indent);

                for stmt in while_stmt.body.iter() {
                    stmt.print(indent + 1);
                }
            }
            Stmt::Assign(assign_stmt) => {
                display("AssignStmt", indent);
                display(" var:", indent);
                display(&assign_stmt.var, indent + 1);
                display(" val:", indent);
                assign_stmt.val.print(indent + 1);
            }
            Stmt::Decl(DeclStmt { var, val }) => {
                display("DeclStmt", indent);
                display(" var:", indent);
                display(var, indent + 1);
                display(" val:", indent);
                val.print(indent + 1);
            }
        }
    }
}

impl TreePrint for Expr {
    fn print(&self, indent: usize) {
        match self {
            Expr::Identfier(name) => {
                display(format!("Identifier('{}')", name), indent)
            }
            Expr::NumberLiteral(value) => {
                display(format!("NumberLiteral({})", value), indent)
            }
            Expr::BooleanLiteral(value) => {
                display(format!("BooleanLiteral({})", value), indent)
            }
            Expr::StringLiteral(value) => {
                display(format!("StringLiteral({})", value), indent)
            }
            Expr::NullLiteral => display("NullLiteral", indent),
            Expr::FnCall(fn_call) => {
                display("FnCall", indent);
                display(" name:", indent);
                display(&fn_call.name, indent + 1);
                display(" args:", indent);

                for arg in fn_call.args.iter() {
                    arg.print(indent + 1);
                }
            }
            Expr::Binary(binary) => {
                display("Binary", indent);
                display(" left:", indent);
                binary.left.print(indent + 1);
                display(" op:", indent);
                display(format!("{:?}", binary.op), indent + 1);
                display(" right:", indent);
                binary.right.print(indent + 1);
            }
            Expr::Unary(unary) => {
                display("Unary", indent);
                display(" expr:", indent);
                unary.expr.print(indent + 1);
                display(" op:", indent);
                display(format!("{:?}", unary.op), indent + 1);
            }
            Expr::ObjectLiteral(fields) => {
                display("ObjectLiteral", indent);

                for (name, value) in fields {
                    display(" name:", indent);
                    display(name, indent + 1);
                    display(" value: ", indent);
                    value.print(indent + 1);
                }
            }
            Expr::ListLiteral(values) => {
                display("ListLiteral", indent);

                for value in values {
                    display(" value: ", indent);
                    value.print(indent + 1);
                }
            }
            Expr::FieldAccess(_) => todo!(),
        }
    }
}

impl TreePrint for Decl {
    fn print(&self, indent: usize) {
        match self {
            Decl::FnDecl(fn_decl) => {
                display("FnDecl", indent);
                display(" name:", indent);
                display(&fn_decl.name, indent + 1);
                display(" params:", indent);

                for param in fn_decl.params.iter() {
                    display(param, indent + 1);
                }

                display(" body:", indent);

                for stmt in fn_decl.body.iter() {
                    stmt.print(indent + 1);
                }
            }
        }
    }
}
