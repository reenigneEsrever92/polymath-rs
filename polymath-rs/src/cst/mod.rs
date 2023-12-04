//!
//! # CST
//!
//! This module contains parser for transforming a stream of asciimath tokens
//! into a Concrete Syntax Tree.
//!
//! ## Asciimath grammar
//!
//! This grammar has been taken from the asciimath website.
//!
//! E ::= IE | I/I                                                      Expression
//! I ::= S_S | S^S | S_S^S | S                                         Intermediate expression
//! S ::= v | lEr | uS | bSS                                            Simple expression
//! v ::= [A-Za-z] | greek letters | numbers | other constant symbols
//! u ::= sqrt | text | bb | other unary symbols for font commands
//! b ::= frac | root | stackrel | other binary symbols
//! l ::= ( | [ | { | (: | {: | other left brackets
//! r ::= ) | ] | } | :) | :} | other right brackets
//!
//! There are two challanges for parsing this particular grammar. For one
//! this grammar is ambigious and second it is non deterministic.
//! I am gonna try a few different techniques to solve those problems.
//! So the is eventually going to be a parser that parses a modified version
//! of the grammar and one that uses lookahead to solve this.
//!
//! In both cases I want to circumvent back tracking entirely.
//!

use std::cell::RefCell;

use crate::tokens::{types::TokenType, Token};

pub mod predictive;

pub type TokenStream<'a> = &'a [Token<'a>];

#[derive(Debug)]
pub struct Cursor {
    pos: RefCell<usize>,
}

impl Cursor {
    fn get_pos(&self) -> usize {
        *self.pos.borrow()
    }

    fn set_pos(&self, pos: usize) -> usize {
        *self.pos.borrow_mut() = pos;
        self.get_pos()
    }

    fn parse<'a, F: Fn(&'a TokenType) -> bool + 'static>(
        &self,
        tokens: TokenStream<'a>,
        func: F,
    ) -> Option<&'a Token<'a>> {
        if let Some(token) = self.peek(tokens, func) {
            self.advance(tokens);
            Some(token)
        } else {
            None
        }
    }

    fn advance<'a>(&self, tokens: TokenStream<'a>) -> Option<&'a Token<'a>> {
        self.set_pos(self.get_pos() + 1);
        tokens.get(self.get_pos() - 1)
    }

    fn slice_to<'a>(
        &self,
        tokens: TokenStream<'a>,
        token: &Token<'a>,
    ) -> Option<(Cursor, TokenStream<'a>)> {
        let token_pos = tokens
            .iter()
            .enumerate()
            .find(|(_, tk)| *tk == token)
            .map(|(index, _)| index);

        token_pos.map(|token_pos| {
            (
                Cursor {
                    pos: RefCell::new(0),
                },
                &tokens[self.get_pos()..token_pos],
            )
        })
    }

    fn peek<'a, F: Fn(&'a TokenType) -> bool>(
        &self,
        tokens: TokenStream<'a>,
        func: F,
    ) -> Option<&'a Token<'a>> {
        self.peek_n(tokens, 0, func)
    }

    fn peek_n<'a, F: Fn(&'a TokenType) -> bool>(
        &self,
        tokens: TokenStream<'a>,
        offset: usize,
        func: F,
    ) -> Option<&'a Token<'a>> {
        tokens
            .get(self.get_pos() + offset)
            .filter(|token| (func)(&token.token_type))
    }

    fn tokens_left(&self, tokens: TokenStream) -> usize {
        tokens.len() - self.get_pos()
    }

    fn eos(&self, tokens: TokenStream) -> bool {
        self.get_pos() >= tokens.len()
    }
}
