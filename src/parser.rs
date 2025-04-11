use crate::lexer::*;
use crate::syntax::*;
use crate::value::Operator;

#[derive(Debug)]
pub enum ParseError {
    ExpectedToken(TokenKind, TokenKind), // expected, received
    EndOfFile,
}

const DEBUG_ENABLED: bool = false;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn debug(&self, msg: impl Into<String>) {
        if DEBUG_ENABLED {
            println!("- {}", msg.into());
        }
    }

    fn current(&self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            Some(&self.tokens[self.position])
        } else {
            None
        }
    }

    fn peek(&self, offset: isize) -> Option<&Token> {
        let index = self.position as isize + offset;
        self.tokens.get(index as usize)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn consume(&mut self, kind: TokenKind) -> Result<&Token, ParseError> {
        if self.current().is_none() {
            return Err(ParseError::EndOfFile);
        };

        self.advance();
        let current = self.peek(-1).unwrap();

        if kind == current.kind {
            Ok(current)
        } else {
            Err(ParseError::ExpectedToken(kind, current.kind))
        }
    }

    pub fn parse(mut self) -> Result<Vec<Decl>, ParseError> {
        if DEBUG_ENABLED {
            println!("PARSE DEBUGGER");
        }

        let mut decls = vec![];

        while let Some(current) = self.current() {
            if let TokenKind::EndOfFile = current.kind {
                break;
            }

            decls.push(self.parse_decl()?);
        }

        Ok(decls)
    }

    fn parse_decl(&mut self) -> Result<Decl, ParseError> {
        self.debug("parse decl");

        let current = self.current().ok_or(ParseError::EndOfFile)?;

        match current.text.as_str() {
            "fn" => Ok(Decl::FnDecl(self.parse_fn_decl()?)),
            _ => panic!("Unknown declaration type: {}", current.text),
        }
    }

    fn parse_fn_decl(&mut self) -> Result<FnDecl, ParseError> {
        self.debug("parse fn decl");

        self.consume(TokenKind::FnKeyword)?;
        let name = self.consume(TokenKind::Identifer)?.text.clone();
        self.consume(TokenKind::LeftParen)?;
        let params = self.parse_params()?;
        self.consume(TokenKind::RightParen)?;
        self.consume(TokenKind::LeftBrace)?;
        let body = self.parse_body()?;
        self.consume(TokenKind::RightBrace)?;

        Ok(FnDecl { name, params, body })
    }

    fn parse_params(&mut self) -> Result<Vec<String>, ParseError> {
        self.debug("parse params");

        let mut params = vec![];

        if let Some(current) = self.current() {
            if let TokenKind::RightParen = current.kind {
                return Ok(params);
            }
        }

        while self.current().is_some() {
            let name = self.consume(TokenKind::Identifer)?.text.clone();

            params.push(name);

            if let Some(TokenKind::RightParen) =
                self.current().map(|token| token.kind)
            {
                break;
            } else {
                self.consume(TokenKind::Comma)?;
            }
        }

        Ok(params)
    }

    fn parse_body(&mut self) -> Result<Vec<Stmt>, ParseError> {
        self.debug("parse body");

        let mut stmts = vec![];

        while let Some(current) = self.current() {
            if let TokenKind::RightBrace = current.kind {
                break;
            }

            stmts.push(self.parse_stmt()?);
        }

        Ok(stmts)
    }

    fn parse_expr_list(&mut self) -> Result<Vec<Expr>, ParseError> {
        self.debug("parse expr list");

        let mut exprs = vec![];

        if let Some(current) = self.current() {
            if let TokenKind::RightParen = current.kind {
                return Ok(exprs);
            }
        }

        while self.current().is_some() {
            exprs.push(self.parse_expr()?);

            if let Some(TokenKind::RightParen) =
                self.current().map(|token| token.kind)
            {
                break;
            } else {
                self.consume(TokenKind::Comma)?;
            }
        }

        Ok(exprs)
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.debug("parse expr");

        self.parse_binary_expr(0)
    }

    fn parse_binary_expr(
        &mut self,
        parent_prec: usize,
    ) -> Result<Expr, ParseError> {
        self.debug("parse binary expr");

        let mut left = self.parse_primary_expr()?;

        loop {
            let Some(current) = self.current() else { break };

            let Ok(op) = Operator::try_from(current.kind) else {
                break;
            };

            let prec = op.get_prec();

            if prec == 0 || prec <= parent_prec {
                break;
            }

            self.consume(current.kind)?;
            let right = self.parse_binary_expr(prec)?;

            left = Expr::Binary(Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    fn parse_primary_expr(&mut self) -> Result<Expr, ParseError> {
        self.debug("parse primary expr");

        let current = self.current().ok_or(ParseError::EndOfFile)?;

        match current.kind {
            TokenKind::Number => {
                let arg = self.consume(TokenKind::Number)?.text.clone();
                let value = arg.parse::<usize>().unwrap();
                Ok(Expr::NumberLiteral(value))
            }
            TokenKind::String => {
                let str = self.consume(TokenKind::String)?.text.clone();
                Ok(Expr::StringLiteral(str[1..str.len() - 1].to_owned()))
            }
            TokenKind::TrueKeyword => {
                self.consume(TokenKind::TrueKeyword)?;
                Ok(Expr::BooleanLiteral(true))
            }
            TokenKind::FalseKeyword => {
                self.consume(TokenKind::FalseKeyword)?;
                Ok(Expr::BooleanLiteral(false))
            }
            TokenKind::Identifer => {
                let ident = current.text.clone();

                if self.peek(1).unwrap().kind == TokenKind::LeftParen {
                    Ok(Expr::FnCall(self.parse_fn_call()?))
                } else {
                    self.consume(TokenKind::Identifer)?;
                    Ok(Expr::Identfier(ident))
                }
            }
            TokenKind::NullKeyword => Ok(Expr::NullLiteral),
            TokenKind::LeftBrace => {
                self.consume(TokenKind::LeftBrace)?;
                let fields = self.parse_object_fields()?;
                self.consume(TokenKind::RightBrace)?;
                Ok(Expr::ObjectLiteral(fields))
            }
            _ => Ok(Expr::FnCall(self.parse_fn_call()?)),
        }
    }

    fn parse_object_fields(
        &mut self,
    ) -> Result<Vec<(String, Expr)>, ParseError> {
        let mut fields = vec![];

        let Some(current) = self.current() else {
            return Err(ParseError::EndOfFile);
        };

        if let TokenKind::RightBrace = current.kind {
            return Ok(fields);
        }

        let name = self.consume(TokenKind::Identifer)?.text.clone();
        self.consume(TokenKind::Colon)?;
        let expr = self.parse_expr()?;

        fields.push((name, expr));

        while self.current().unwrap().kind != TokenKind::RightBrace {
            self.consume(TokenKind::Comma)?;

            let name = self.consume(TokenKind::Identifer)?.text.clone();
            self.consume(TokenKind::Colon)?;
            let expr = self.parse_expr()?;

            fields.push((name, expr));
        }

        Ok(fields)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.debug("parse stmt");

        let Some(current) = self.current() else {
            return Err(ParseError::EndOfFile);
        };

        match current.kind {
            TokenKind::ReturnKeyword => {
                Ok(Stmt::Return(self.parse_return_stmt()?))
            }
            TokenKind::IfKeyword => Ok(Stmt::If(self.parse_if_stmt()?)),
            TokenKind::LetKeyword => {
                self.consume(TokenKind::LetKeyword)?;
                let var = self.consume(TokenKind::Identifer)?.text.clone();
                self.consume(TokenKind::Equals)?;
                let val = self.parse_expr()?;
                self.consume(TokenKind::Semicolon)?;
                Ok(Stmt::Decl(DeclStmt { var, val }))
            }
            _ => {
                let next = self.peek(1).ok_or(ParseError::EndOfFile)?;

                if next.kind == TokenKind::LeftParen {
                    let stmt = Stmt::FnCall(self.parse_fn_call()?);
                    self.consume(TokenKind::Semicolon)?;
                    Ok(stmt)
                } else {
                    let stmt = Stmt::Assign(self.parse_assign()?);
                    self.consume(TokenKind::Semicolon)?;
                    Ok(stmt)
                }
            }
        }
    }

    fn parse_assign(&mut self) -> Result<AssignStmt, ParseError> {
        let var = self.consume(TokenKind::Identifer)?.text.clone();
        self.consume(TokenKind::Equals)?;
        let val = self.parse_expr()?;

        Ok(AssignStmt { var, val })
    }

    fn parse_if_stmt(&mut self) -> Result<IfStmt, ParseError> {
        self.debug("parse if stmt");

        self.consume(TokenKind::IfKeyword)?;
        let cond = self.parse_expr()?;
        self.consume(TokenKind::LeftBrace)?;
        let body = self.parse_body()?;
        self.consume(TokenKind::RightBrace)?;

        Ok(IfStmt { cond, body })
    }

    fn parse_return_stmt(&mut self) -> Result<ReturnStmt, ParseError> {
        self.debug("parse return stmt");

        self.consume(TokenKind::ReturnKeyword)?;

        let Some(current) = self.current() else {
            return Err(ParseError::EndOfFile);
        };

        let stmt = if let TokenKind::Semicolon = current.kind {
            ReturnStmt {
                expr: Expr::NullLiteral,
            }
        } else {
            ReturnStmt {
                expr: self.parse_expr()?,
            }
        };

        self.consume(TokenKind::Semicolon)?;

        Ok(stmt)
    }

    fn parse_fn_call(&mut self) -> Result<FnCall, ParseError> {
        self.debug("parse fn call");

        let name = self.consume(TokenKind::Identifer)?.text.clone();
        self.consume(TokenKind::LeftParen)?;
        let args = self.parse_expr_list()?;
        self.consume(TokenKind::RightParen)?;

        Ok(FnCall { name, args })
    }
}
