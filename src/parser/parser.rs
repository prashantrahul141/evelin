use log::debug;

use crate::ast::{
    {FnDecl, StructDecl}, {Token, TokenType},
};

pub struct Parser<'a> {
    /// Vec of tokens to parse.
    pub(super) tokens: &'a Vec<Token>,

    /// Current token position.
    pub(super) current: usize,

    /// flag to be set if any error occurs during parsing.
    pub has_errors: bool,

    /// vec of all parsed struct declarations.
    pub struct_decls: Vec<StructDecl>,

    /// vec of all parsed function declarations.
    pub fn_decls: Vec<FnDecl>,
}

impl<'a> From<&'a Vec<Token>> for Parser<'a> {
    fn from(tokens: &'a Vec<Token>) -> Self {
        debug!("Start parsing--------------------------------");
        Self {
            tokens,
            current: 0,
            has_errors: false,
            struct_decls: vec![],
            fn_decls: vec![],
        }
    }
}

impl<'a> Parser<'a> {
    /// Public api to start parsing.
    ///
    pub fn parse(&mut self) {
        while !self.is_at_end() {
            if self.match_token(&[TokenType::Struct]) {
                match self.struct_decl() {
                    Ok(decl) => self.struct_decls.push(decl),
                    Err(e) => todo!("{:?}", e),
                };
            } else if self.match_token(&[TokenType::Fn]) {
                match self.fn_decl() {
                    Ok(decl) => self.fn_decls.push(decl),
                    Err(e) => todo!("{:?}", e),
                };
            } else {
                self.report_parser_error("Expected function or struct declaration.");
            }
        }
    }
}
