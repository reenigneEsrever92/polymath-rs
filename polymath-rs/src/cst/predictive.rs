//!
//! # Predictive
//!
//! This parser is going to utilise lookahead to predict the applicable
//! production rule.
//!
//! One modification will still be made to the grammar: An Expression
//! can be empty. Otherwise the parser would never halt since E is defined
//! as:
//!
//! E ::= I/I | IE <- there has to always be at least one more character
//!
//! So E becomes:
//!
//! E ::= I/E | IE | ε
//!
//! `I/I` also became `I/E` otherwise the parser would stop consuming after `I`
//! so that nothing further will be processed after a fraction. Yet, All `E`s
//! coming after the next I can't be part of the fraction which they are due to
//! this change. This will be corrected in CST to AST conversion though.
//!

use std::cell::RefCell;

use crate::tokens::{types::TokenType, Span};

use super::{Cursor, Token, TokenStream};

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct PredictiveCST<'a> {
    pub expression: Expression<'a>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Expression<'a> {
    // IE
    IE(IntermediateExpression<'a>, Box<Expression<'a>>),
    // I/I
    II(IntermediateExpression<'a>, Token<'a>, Box<Expression<'a>>),
    Unit,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum IntermediateExpression<'a> {
    // S_S
    SimpleSub(SimpleExpression<'a>, Token<'a>, SimpleExpression<'a>),
    // S^S
    SimpleSup(SimpleExpression<'a>, Token<'a>, SimpleExpression<'a>),
    // S_S^S
    SimpleSubSup(
        SimpleExpression<'a>,
        Token<'a>,
        SimpleExpression<'a>,
        Token<'a>,
        SimpleExpression<'a>,
    ),
    // S
    Simple(SimpleExpression<'a>),
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum SimpleExpression<'a> {
    // v
    Symbol(Token<'a>),
    // lEr
    Group(Token<'a>, Box<Expression<'a>>, Token<'a>),
    // uS
    UnarySymbol(Token<'a>, Box<SimpleExpression<'a>>),
    // bSS
    BinarySymbol(
        Token<'a>,
        Box<SimpleExpression<'a>>,
        Box<SimpleExpression<'a>>,
    ),
}

///
/// Parses a stream of tokes according to the grammar.
///
pub fn parse(tokens: TokenStream) -> PredictiveCST {
    let cursor = Cursor {
        pos: RefCell::new(0),
    };
    if cursor.eos(tokens) {
        PredictiveCST {
            expression: Expression::Unit,
        }
    } else {
        PredictiveCST {
            expression: parse_expression(tokens, &cursor),
        }
    }
}

fn parse_expression<'a>(tokens: TokenStream<'a>, cursor: &Cursor) -> Expression<'a> {
    let intermediate_expression = parse_intermediate_expression(tokens, cursor);
    // test for division
    if let Some(division_token) = cursor.parse(tokens, |tt| {
        matches!(tt, crate::tokens::types::TokenType::Division)
    }) {
        Expression::II(
            intermediate_expression,
            division_token.clone(),
            Box::new(parse_expression(tokens, cursor)),
        )
    } else if cursor.eos(tokens) {
        Expression::IE(intermediate_expression, Box::new(Expression::Unit))
    } else {
        Expression::IE(
            intermediate_expression,
            Box::new(parse_expression(tokens, cursor)),
        )
    }
}

fn parse_intermediate_expression<'a>(
    tokens: TokenStream<'a>,
    cursor: &Cursor,
) -> IntermediateExpression<'a> {
    let simple_expression = parse_simple_expression(tokens, cursor);

    if let Some(sub) = cursor.parse(tokens, |tt| {
        matches!(tt, crate::tokens::types::TokenType::Underscorce)
    }) {
        let simple_expression_2 = parse_simple_expression(tokens, cursor);

        if let Some(sup) = cursor.parse(tokens, |tt| {
            matches!(tt, crate::tokens::types::TokenType::Hat)
        }) {
            IntermediateExpression::SimpleSubSup(
                simple_expression,
                sub.clone(),
                simple_expression_2,
                sup.clone(),
                parse_simple_expression(tokens, cursor),
            )
        } else {
            IntermediateExpression::SimpleSub(simple_expression, sub.clone(), simple_expression_2)
        }
    } else if let Some(sup) = cursor.parse(tokens, |tt| {
        matches!(tt, crate::tokens::types::TokenType::Hat)
    }) {
        IntermediateExpression::SimpleSup(
            simple_expression,
            sup.clone(),
            parse_simple_expression(tokens, cursor),
        )
    } else {
        IntermediateExpression::Simple(simple_expression)
    }
}

fn parse_simple_expression<'a>(tokens: TokenStream<'a>, cursor: &Cursor) -> SimpleExpression<'a> {
    if let Some(simple_expression) = parse_group(tokens, cursor) {
        simple_expression
    } else if let Some(unary_symbol) = cursor.parse(tokens, |tt| {
        matches!(tt, crate::tokens::types::TokenType::UnaryOperator(_))
    }) {
        SimpleExpression::UnarySymbol(
            unary_symbol.clone(),
            Box::new(parse_simple_expression(tokens, cursor)),
        )
    } else if let Some(binary_symbol) = cursor.parse(tokens, |tt| {
        matches!(tt, crate::tokens::types::TokenType::BinaryOperator(_))
    }) {
        SimpleExpression::BinarySymbol(
            binary_symbol.clone(),
            Box::new(parse_simple_expression(tokens, cursor)),
            Box::new(parse_simple_expression(tokens, cursor)),
        )
    } else if let Some(token) = cursor.parse(tokens, |_| true) {
        SimpleExpression::Symbol(token.clone())
    } else {
        SimpleExpression::Symbol(Token {
            span: Span {
                start: cursor.get_pos(),
                end: cursor.get_pos(),
                text: "",
            },
            token_type: TokenType::None,
        })
    }
}

fn parse_group<'a>(tokens: TokenStream<'a>, cursor: &Cursor) -> Option<SimpleExpression<'a>> {
    if cursor
        .peek(tokens, |tt| {
            matches!(tt, crate::tokens::types::TokenType::LBrace(_))
        })
        .is_some()
    {
        let mut group_count = 1;
        let mut closing_bracket = None;

        for i in 1..cursor.tokens_left(tokens) {
            let opening_parens = cursor.peek_n(tokens, i, |tt| {
                matches!(tt, crate::tokens::types::TokenType::LBrace(_))
            });
            let closing_parens = cursor.peek_n(tokens, i, |tt| {
                matches!(tt, crate::tokens::types::TokenType::RBrace(_))
            });

            match (opening_parens, closing_parens) {
                (None, Some(_)) => group_count -= 1,
                (Some(_), None) => group_count += 1,
                _ => {}
            }

            if group_count == 0 {
                closing_bracket = closing_parens;
                break;
            }
        }

        if let Some(closing_bracket) = closing_bracket {
            let lbrace = cursor.advance(tokens).unwrap();
            let (group_cursor, group_tokens) = cursor.slice_to(tokens, closing_bracket).unwrap();

            cursor.set_pos(cursor.get_pos() + group_tokens.len());

            Some(SimpleExpression::Group(
                lbrace.clone(),
                Box::new(parse_expression(group_tokens, &group_cursor)),
                cursor.advance(tokens).unwrap().clone(),
            ))
        } else {
            None
        }
    } else {
        None
    }
}
