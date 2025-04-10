use std::panic;

use evelin::ast::{BinOp, Expr, FnDecl, LiteralValue, Stmt, StructDecl, TokenType};
use evelin::lexer::Lexer;
use evelin::parser::Parser;

fn tokenize<T: Into<String>>(input: T) -> Vec<evelin::lexer::Token> {
    let l = input.into();
    let mut lexer = Lexer::from(&l);
    lexer.start();
    let t = lexer.tokens();
    (*t).clone()
}

fn parse_fn(source: &str) -> Vec<FnDecl> {
    let tokens = tokenize(source.to_string());
    let mut parser = Parser::from(&tokens);
    parser.parse();
    parser.fn_decls
}

fn parser_struct(source: &str) -> Vec<StructDecl> {
    let tokens = tokenize(source.to_string());
    let mut parser = Parser::from(&tokens);
    parser.parse();
    parser.struct_decls
}

#[test]
fn parses_empty_struct() {
    let parser = parser_struct("struct Foo {}");

    assert_eq!(parser.len(), 1);
    let s = &parser[0];
    assert_eq!(s.name, "Foo");
    assert!(s.fields.is_empty());
}

#[test]
fn parses_struct_with_fields() {
    let parser = parser_struct("struct Point { x: i64, y: f64 }");

    assert_eq!(parser.len(), 1);
    let s = &parser[0];
    assert_eq!(s.name, "Point");
    assert_eq!(
        s.fields,
        vec![
            ("x".to_string(), TokenType::TypeI64),
            ("y".to_string(), TokenType::TypeF64)
        ]
    );
}

#[test]
fn parses_function_without_param() {
    let parser = parse_fn("fn main() -> void { return 42; }");

    assert_eq!(parser.len(), 1);
    let f = &parser[0];
    assert_eq!(f.name, "main");
    assert!(f.parameter.is_none());
    assert_eq!(f.return_type, TokenType::TypeVoid);
    assert!(!f.body.is_empty());
}

#[test]
fn parses_function_with_param() {
    let parser = parse_fn("fn inc(x: i64) -> i64 { return x; }");

    assert_eq!(parser.len(), 1);
    let f = &parser[0];
    assert_eq!(f.name, "inc");
    assert_eq!(
        f.parameter.as_ref(),
        Some(&("x".to_owned(), TokenType::TypeI64))
    );
    assert_eq!(f.return_type, TokenType::TypeI64);
}

#[test]
fn parses_if_else_statement() {
    let parser = parse_fn("fn test() -> i64 { if (true) { print 1; } else { print 2; } }");

    let body = &parser[0].body;
    assert!(matches!(body[0], Stmt::If(_)));
}

#[test]
fn parses_literal_expression() {
    let parser = parse_fn("fn test() -> i64 { return 123; }");

    if let Stmt::Return(ret_stmt) = &parser[0].body[0] {
        match &ret_stmt.value {
            Expr::Literal(lit) => {
                matches!(lit.value, LiteralValue::NumberInt(123));
            }
            _ => panic!("Expected literal int"),
        }
    } else {
        panic!("Expected return stmt");
    }
}

#[test]
fn parses_binary_expression() {
    let parser = parse_fn("fn test() -> i64 { return 1 + 2 * 3; }");

    if let Stmt::Return(ret_stmt) = &parser[0].body[0] {
        match &ret_stmt.value {
            Expr::Binary(bin) => {
                matches!(bin.op, BinOp::Add);
            }
            _ => panic!("Expected binary expression"),
        }
    } else {
        panic!("Expected return stmt");
    }
}

#[test]
fn parses_nested_blocks() {
    let parser = parse_fn("fn test() -> i64 { { { print 1; } } }");

    assert_eq!(parser.len(), 1);
    let outer_block = &parser[0].body[0];
    assert!(matches!(outer_block, Stmt::Block(_)));
}

#[test]
fn parses_call_without_arg() {
    let parser = parse_fn("fn main() -> i64 { main(); }");

    assert_eq!(parser.len(), 1);
    let block = &parser[0].body[0];
    match block {
        Stmt::Expression(stmt) => match stmt {
            Expr::Call(call) => {
                match call.callee.clone() {
                    Expr::Variable(var) => {
                        assert_eq!(var.name, "main".to_string());
                    }
                    _ => panic!("Expected Expr::Variable"),
                }

                assert!(call.arg.is_none());
            }
            _ => panic!("Expressed call expression."),
        },
        _ => panic!("Expected expression stmt."),
    }
}

#[test]
fn parses_call_with_arg() {
    let parser = parse_fn("fn main() -> i64 { main(1 + 1); }");

    assert_eq!(parser.len(), 1);
    let block = &parser[0].body[0];
    match block {
        Stmt::Expression(stmt) => match stmt {
            Expr::Call(call) => {
                match call.callee.clone() {
                    Expr::Variable(var) => {
                        assert_eq!(var.name, "main".to_string());
                    }
                    _ => panic!("Expected Expr::Variable"),
                }

                match call.arg.clone() {
                    Some(arg) => match arg {
                        Expr::Binary(bin) => {
                            match bin.left {
                                Expr::Literal(_) => {}
                                _ => panic!("Expected literal"),
                            }

                            matches!(bin.op, BinOp::Add);
                            match bin.right {
                                Expr::Literal(_) => {}
                                _ => panic!("Expected literal"),
                            }
                        }
                        _ => panic!("Should have been a binary expr"),
                    },
                    None => panic!("arg is none."),
                }
            }
            _ => panic!("Expressed call expression."),
        },
        _ => panic!("Expected expression stmt."),
    }
}

#[test]
fn parses_native_call_without_arg() {
    let parser = parse_fn("fn main() -> i64 { extern main(); }");

    assert_eq!(parser.len(), 1);
    let block = &parser[0].body;
    match block.first().unwrap() {
        Stmt::Expression(stmt) => match stmt {
            Expr::NativeCall(call) => {
                match call.callee.clone() {
                    Expr::Variable(var) => {
                        assert_eq!(var.name, "main".to_string());
                    }
                    _ => panic!("Expected Expr::Variable"),
                }

                assert_eq!(call.args.len(), 0);
            }
            _ => panic!("Expressed call expression."),
        },
        _ => panic!("Expected expression stmt."),
    }
}
