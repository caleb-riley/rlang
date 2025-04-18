use std::fs;

use interpreter::*;
use lexer::*;
use parser::*;
use value::OperationError;

mod interpreter;
mod lexer;
mod parser;
mod printing;
mod scope;
mod syntax;
mod value;

fn report_parse_err(msg: impl Into<String>) -> ! {
    panic!("Parse error: {}", msg.into());
}

fn report_runtime_err(msg: impl Into<String>) -> ! {
    panic!("Runtime error: {}", msg.into());
}

fn main() -> std::io::Result<()> {
    let source = match std::env::args().collect::<Vec<_>>().get(1) {
        Some(path) => fs::read_to_string(format!("./{}", path))?,
        None => panic!("Must provide a path to the program"),
    };

    let lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    let parser = Parser::new(tokens);

    let decls = match parser.parse() {
        Ok(decls) => decls,
        Err(parse_err) => match parse_err {
            ParseError::EndOfFile => {
                report_parse_err("Expected token but reached end of file");
            }
            ParseError::ExpectedToken(exp, rec) => {
                report_parse_err(format!("Expected {:?}, got {:?}", exp, rec));
            }
        },
    };

    let interpreter = Interpreter::new();

    match interpreter.interpret(decls) {
        Ok(()) => Ok(()),
        Err(run_err) => match run_err {
            RuntimeError::OperationError(op_err) => match op_err {
                OperationError::InvalidBinary(left, op, right) => {
                    report_runtime_err(format!(
                        "Cannot use binary operator {:?} on types {} and {}",
                        op,
                        left.type_name(),
                        right.type_name()
                    ));
                }
                OperationError::InvalidUnary(op, expr) => {
                    report_runtime_err(format!(
                        "Cannot use unary operator {:?} on type {}",
                        op,
                        expr.type_name(),
                    ));
                }
            },
            RuntimeError::InvalidArgCount(exp, rec) => report_runtime_err(
                format!("Expected {} args, got {}", exp, rec),
            ),
            RuntimeError::UndefinedIdentifier(name) => {
                report_runtime_err(format!("Unknown identifier: {}", name))
            }
            RuntimeError::InvalidArgumentType(expected, got) => {
                report_runtime_err(format!(
                    "Expected type {expected}, got {got}"
                ))
            }
            RuntimeError::NoScope => {
                report_runtime_err("All scopes have been popped")
            }
            RuntimeError::IndexOutOfBounds(length, tried) => {
                report_runtime_err(format!(
                    "Index out of bounds: used index {} on a list of length {}",
                    tried, length
                ))
            }
        },
    }
}
