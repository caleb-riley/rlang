use std::cell::RefCell;
use std::collections::HashMap;
use std::io::stdin;
use std::rc::Rc;

use crate::registry::ObjRegistry;
use crate::scope::Scope;
use crate::syntax::*;
use crate::value::*;

pub enum RuntimeError {
    OperationError(OperationError),
    InvalidArgCount(usize, usize),
    UndefinedIdentifier(String),
    InvalidArgumentType(String, String),
}

enum BodyResult {
    Return(Value),
    None,
}

enum FnBody {
    Builtin(Box<dyn Fn(Vec<Value>) -> Result<Value, RuntimeError> + 'static>),
    Defined(FnDecl),
}

pub struct FnObj {
    params: Vec<String>,
    body: FnBody,
}

pub struct Interpreter {
    decls: Vec<Decl>,
    scope: Rc<RefCell<Scope>>,
    registry: ObjRegistry,
}

impl Interpreter {
    pub fn new(decls: Vec<Decl>) -> Self {
        Self {
            decls,
            scope: Rc::new(RefCell::new(Scope::new())),
            registry: ObjRegistry::new(),
        }
    }

    fn define_fn(
        &mut self,
        name: &str,
        params: Vec<&str>,
        body: impl Fn(Vec<Value>) -> Result<Value, RuntimeError> + 'static,
    ) {
        self.registry.register_func(
            name.to_owned(),
            FnObj {
                params: params.into_iter().map(|s| s.to_owned()).collect(),
                body: FnBody::Builtin(Box::new(body)),
            },
        );
    }

    fn define_builtins(&mut self) {
        self.define_fn("print", vec!["val"], |args| {
            println!("{}", args[0]);
            Ok(Value::Null)
        });

        self.define_fn("input", vec![], |_| {
            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            Ok(Value::String(buf.trim_end().to_owned()))
        });

        self.define_fn("parseint", vec!["str"], |args| match args[0] {
            Value::String(ref str) => {
                Ok(Value::Number(str.parse::<usize>().unwrap()))
            }
            _ => Err(RuntimeError::InvalidArgumentType(
                "string".into(),
                args[0].type_name().into(),
            )),
        });

        self.define_fn("tostring", vec!["val"], |args| {
            Ok(Value::String(args[0].to_string()))
        });

        let scope = Rc::clone(&self.scope);

        self.define_fn("define", vec!["name", "val"], move |mut args| {
            let name = args.remove(0);

            let Value::String(text) = name else {
                return Err(RuntimeError::InvalidArgumentType(
                    "string".into(),
                    name.type_name().into(),
                ));
            };

            scope.borrow_mut().declare(text.clone(), args.remove(0));

            Ok(Value::Null)
        });
    }

    pub fn interpret(mut self) -> Result<(), RuntimeError> {
        self.define_builtins();

        for decl in self.decls.clone() {
            self.interpret_decl(decl);
        }

        let cmd_args = std::env::args()
            .skip(2)
            .map(Value::String)
            .collect::<Vec<_>>();

        match self.registry.get_func(&"main".to_owned()) {
            Some(main) => self.call_fn(main, cmd_args)?,
            None => panic!("No main function found!"),
        };

        Ok(())
    }

    fn interpret_body(
        &self,
        body: &[Stmt],
    ) -> Result<BodyResult, RuntimeError> {
        self.scope.borrow_mut().push_scope();

        for stmt in body.iter() {
            if let BodyResult::Return(val) = self.interpret_stmt(stmt)? {
                self.scope.borrow_mut().pop_scope();
                return Ok(BodyResult::Return(val));
            }
        }

        self.scope.borrow_mut().pop_scope();

        Ok(BodyResult::Return(Value::Null))
    }

    fn call_fn(
        &self,
        func: &FnObj,
        args: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        if args.len() != func.params.len() {
            return Err(RuntimeError::InvalidArgCount(
                args.len(),
                func.params.len(),
            ));
        }

        match &func.body {
            FnBody::Builtin(builtin_fn) => builtin_fn(args),
            FnBody::Defined(defined_fn) => {
                self.scope.borrow_mut().push_scope();

                for (index, arg) in args.iter().enumerate() {
                    self.scope
                        .borrow_mut()
                        .declare(func.params[index].clone(), arg.clone());
                }

                let res =
                    self.interpret_body(&defined_fn.body).map(|body_res| {
                        match body_res {
                            BodyResult::Return(val) => val,
                            BodyResult::None => Value::Null,
                        }
                    });

                self.scope.borrow_mut().pop_scope();

                res
            }
        }
    }

    fn interpret_decl(&mut self, decl: Decl) {
        match decl {
            Decl::FnDecl(fn_decl) => {
                self.registry.register_func(
                    fn_decl.name.clone(),
                    FnObj {
                        params: fn_decl.params.clone(),
                        body: FnBody::Defined(fn_decl),
                    },
                );
            }
        }
    }

    fn interpret_stmt(&self, stmt: &Stmt) -> Result<BodyResult, RuntimeError> {
        match stmt {
            Stmt::FnCall(FnCall { name, args }) => {
                let func = self
                    .registry
                    .get_func(name)
                    .ok_or(RuntimeError::UndefinedIdentifier(name.clone()))?;
                self.call_fn(
                    func,
                    args.iter()
                        .map(|arg| self.evaluate(arg))
                        .collect::<Result<Vec<_>, _>>()?,
                )?;
                Ok(BodyResult::None)
            }
            Stmt::If(IfStmt { cond, body }) => {
                let Value::Boolean(cond_val) = self.evaluate(cond)? else {
                    panic!("IfStmt must have boolean as condition!");
                };

                if cond_val {
                    self.interpret_body(body)
                } else {
                    Ok(BodyResult::None)
                }
            }
            Stmt::Return(ReturnStmt { expr }) => {
                Ok(BodyResult::Return(self.evaluate(expr)?))
            }
            Stmt::Assign(AssignStmt { var, val }) => {
                let val = self.evaluate(val)?;
                self.scope.borrow_mut().set(var.clone(), val);
                Ok(BodyResult::None)
            }
            Stmt::Decl(DeclStmt { var, val }) => {
                let val = self.evaluate(val)?;
                self.scope.borrow_mut().declare(var.clone(), val);
                Ok(BodyResult::None)
            }
        }
    }

    fn evaluate(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Identfier(name) => Ok(self
                .scope
                .borrow()
                .get(name)
                .ok_or(RuntimeError::UndefinedIdentifier(name.clone()))?
                .clone()),
            Expr::NumberLiteral(num) => Ok(Value::Number(*num)),
            Expr::BooleanLiteral(bool) => Ok(Value::Boolean(*bool)),
            Expr::NullLiteral => Ok(Value::Null),
            Expr::StringLiteral(str) => Ok(Value::String(str.clone())),
            Expr::FnCall(FnCall { name, args }) => {
                let func = self
                    .registry
                    .get_func(name)
                    .ok_or(RuntimeError::UndefinedIdentifier(name.clone()))?;
                let res = self.call_fn(
                    func,
                    args.iter()
                        .map(|arg| self.evaluate(arg))
                        .collect::<Result<Vec<_>, _>>()?,
                )?;
                Ok(res)
            }
            Expr::Binary(bin_expr) => {
                let left = self.evaluate(&bin_expr.left)?;
                let right = self.evaluate(&bin_expr.right)?;

                Ok(left
                    .operate(&right, bin_expr.op.clone())
                    .map_err(RuntimeError::OperationError))?
            }
            Expr::ObjectLiteral(fields) => {
                let mut object = HashMap::new();

                for (name, expr) in fields {
                    object.insert(name.clone(), self.evaluate(expr)?);
                }

                Ok(Value::Object(object))
            }
            Expr::FieldAccess(FieldAccess { .. }) => todo!(),
        }
    }
}
