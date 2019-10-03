#![allow(dead_code)]

use super::Error;
use super::Result;
use super::super::collections::{
    token::{ Token, TokenType },
    parse_tree::*
};

use std::iter::Peekable;

/// A struct which encapsulates the state of the parser.
pub struct Parser<'a, I> where
        I : Iterator<Item = Result<Token<'a>>> {
    scanner : Peekable<I>,
    row : usize,
    column : usize
}
impl<'a, I> Parser<'a, I> where
        I : Iterator<Item = Result<Token<'a>>> {
    /// Create a new parser using this scanner.
    pub fn from(scanner : I) -> Parser<'a, I> {
        Parser {
            scanner : scanner.peekable(),
            row : 0,
            column : 0
        }
    }

    /// Consumes the parser and produces an abstract syntax tree.
    pub fn parse(mut self) -> Result<Expr<'a>> {
        Err(vec![Error {
            description : "not implemented",
            row : 0,
            column : 0
        }])
    }
}

/// A trait which can be implemented by structs to offer a
/// way of converting into an abstract syntax tree.
pub trait Builder<'a> {
    /// Constructs a new scanner.
    fn into_ast(self) -> Result<Expr<'a>>;
}
impl<'a, I> Builder<'a> for I where
        I : Iterator<Item = Result<Token<'a>>> {
    fn into_ast(self) -> Result<Expr<'a>> {
        Parser::from(self).parse()
    }
}