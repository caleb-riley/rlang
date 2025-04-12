use std::cell::RefCell;
use std::collections::HashMap;
use std::io::stdin;
use std::rc::Rc;

use crate::scope::ScopeManager;
use crate::syntax::*;
use crate::value::*;

pub enum RuntimeError {
    OperationError(OperationError),
    InvalidArgCount(usize, usize),
    UndefinedIdentifier(String),
    InvalidArgumentType(String, String),
    NoScope,
}

enum BodyResult {
    Return(Value),
    None,
}

enum FnObj {
    Builtin {
        param_count: usize,
        body: Box<dyn Fn(Vec<Value>) -> Result<Value, RuntimeError> + 'static>,
    },
    Defined {
        params: Vec<String>,
        body: Vec<Stmt>,
    },
}

impl FnObj {
    fn param_count(&self) -> usize {
        match self {
            Self::Builtin { param_count, .. } => *param_count,
            Self::Defined { params, .. } => params.len(),
        }
    }
}

pub struct Interpreter {
    scope: Rc<RefCell<ScopeManager>>,
    funcs: HashMap<String, FnObj>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            scope: Rc::new(RefCell::new(ScopeManager::default())),
            funcs: HashMap::new(),
        }
    }

    fn define_fn(
        &mut self,
        name: &str,
        param_count: usize,
        body: impl Fn(Vec<Value>) -> Result<Value, RuntimeError> + 'static,
    ) {
        self.funcs.insert(
            name.to_owned(),
            FnObj::Builtin {
                param_count,
                body: Box::new(body),
            },
        );
    }

    fn define_builtins(&mut self) {
        self.define_fn("print", 1, |args| {
            println!("{}", args[0]);
            Ok(Value::Null)
        });

        self.define_fn("input", 0, |_| {
            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            Ok(Value::String(buf.trim_end().to_owned()))
        });

        self.define_fn("parseint", 1, |args| match args[0] {
            Value::String(ref str) => {
                Ok(Value::Number(str.parse::<i32>().unwrap()))
            }
            _ => Err(RuntimeError::InvalidArgumentType(
                "string".into(),
                args[0].type_name().into(),
            )),
        });

        self.define_fn("tostring", 1, |args| {
            Ok(Value::String(args[0].to_string()))
        });

        let scope = Rc::clone(&self.scope);

        self.define_fn("define", 2, move |mut args| {
            let name = args.remove(0);

            let Value::String(text) = name else {
                return Err(RuntimeError::InvalidArgumentType(
                    "string".into(),
                    name.type_name().into(),
                ));
            };

            scope
                .borrow_mut()
                .inner_mut()
                .ok_or(RuntimeError::NoScope)?
                .declare(text, args.remove(0));

            Ok(Value::Null)
        });

        let scope = Rc::clone(&self.scope);

        self.define_fn("var", 1, move |mut args| {
            let name = args.remove(0);

            let Value::String(text) = name else {
                return Err(RuntimeError::InvalidArgumentType(
                    "string".into(),
                    name.type_name().into(),
                ));
            };

            Ok(scope
                .borrow_mut()
                .inner()
                .ok_or(RuntimeError::NoScope)?
                .get(&text)
                .cloned()
                .unwrap_or(Value::Null))
        });
    }

    pub fn interpret(mut self, decls: Vec<Decl>) -> Result<(), RuntimeError> {
        self.define_builtins();

        for decl in decls {
            self.interpret_decl(decl);
        }

        let cmd_args = std::env::args()
            .skip(2)
            .map(Value::String)
            .collect::<Vec<_>>();

        match self.funcs.get("main") {
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
            if let ret @ BodyResult::Return(_) = self.interpret_stmt(stmt)? {
                self.scope
                    .borrow_mut()
                    .pop_scope()
                    .map_err(|_| RuntimeError::NoScope)?;

                return Ok(ret);
            }
        }

        self.scope
            .borrow_mut()
            .pop_scope()
            .map_err(|_| RuntimeError::NoScope)?;

        Ok(BodyResult::None)
    }

    fn call_fn(
        &self,
        func: &FnObj,
        args: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        if args.len() != func.param_count() {
            return Err(RuntimeError::InvalidArgCount(
                args.len(),
                func.param_count(),
            ));
        }

        match &func {
            FnObj::Builtin { body, .. } => body(args),
            FnObj::Defined { params, body } => {
                self.scope.borrow_mut().push_scope();

                for (param, arg) in params.iter().zip(args.into_iter()) {
                    self.scope
                        .borrow_mut()
                        .inner_mut()
                        .ok_or(RuntimeError::NoScope)?
                        .declare(param.clone(), arg);
                }

                let res =
                    self.interpret_body(body).map(|body_res| match body_res {
                        BodyResult::Return(val) => val,
                        BodyResult::None => Value::Null,
                    });

                self.scope
                    .borrow_mut()
                    .pop_scope()
                    .map_err(|_| RuntimeError::NoScope)?;

                res
            }
        }
    }

    fn interpret_decl(&mut self, decl: Decl) {
        match decl {
            Decl::FnDecl(fn_decl) => {
                self.funcs.insert(
                    fn_decl.name,
                    FnObj::Defined {
                        params: fn_decl.params,
                        body: fn_decl.body,
                    },
                );
            }
        }
    }

    fn interpret_stmt(&self, stmt: &Stmt) -> Result<BodyResult, RuntimeError> {
        match stmt {
            Stmt::FnCall(FnCall { name, args }) => {
                let func = self
                    .funcs
                    .get(name)
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
            Stmt::While(WhileStmt { cond, body }) => loop {
                let Value::Boolean(result) = self.evaluate(cond)? else {
                    panic!("WhileStmt must have boolean as condition!");
                };

                if !result {
                    return Ok(BodyResult::None);
                }

                if let result @ BodyResult::Return(_) =
                    self.interpret_body(body)?
                {
                    return Ok(result);
                }
            },
            Stmt::Return(ReturnStmt { expr }) => {
                Ok(BodyResult::Return(self.evaluate(expr)?))
            }
            Stmt::Assign(AssignStmt { var, val }) => {
                let val = self.evaluate(val)?;
                self.scope
                    .borrow_mut()
                    .inner_mut()
                    .ok_or(RuntimeError::NoScope)?
                    .set(var, val)
                    .map_err(|_| RuntimeError::NoScope)?;
                Ok(BodyResult::None)
            }
            Stmt::Decl(DeclStmt { var, val }) => {
                let val = self.evaluate(val)?;
                self.scope
                    .borrow_mut()
                    .inner_mut()
                    .ok_or(RuntimeError::NoScope)?
                    .declare(var.clone(), val);
                Ok(BodyResult::None)
            }
        }
    }

    fn evaluate(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Identfier(name) => Ok(self
                .scope
                .borrow()
                .inner()
                .ok_or(RuntimeError::NoScope)?
                .get(name)
                .ok_or(RuntimeError::UndefinedIdentifier(name.clone()))?
                .clone()),
            Expr::NumberLiteral(num) => Ok(Value::Number(*num)),
            Expr::BooleanLiteral(bool) => Ok(Value::Boolean(*bool)),
            Expr::NullLiteral => Ok(Value::Null),
            Expr::StringLiteral(str) => Ok(Value::String(str.clone())),
            Expr::FnCall(FnCall { name, args }) => {
                let func = self
                    .funcs
                    .get(name)
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
                    .operate(&right, bin_expr.op)
                    .map_err(RuntimeError::OperationError))?
            }
            Expr::Unary(unary_expr) => {
                let expr = self.evaluate(&unary_expr.expr)?;

                Ok(expr
                    .operate_unary(unary_expr.op)
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
