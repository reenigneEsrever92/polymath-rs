//!
//! # AST
//!
//! This module contains different CSTs into a unified AST which will be used
//! by various backends (mathml for example).
//!

pub mod predictive;

use std::collections::VecDeque;

use crate::tokens::Token;

#[derive(Debug, Clone)]
pub struct AST<'a> {
    pub expressions: Expressions<'a>,
}

#[derive(Debug, Clone)]
pub struct Expressions<'a> {
    pub expressions: VecDeque<Expression<'a>>,
}

#[derive(Debug, Clone)]
pub enum Expression<'a> {
    Frac(BiExpression<'a>),
    Sub(BiExpression<'a>),
    Pow(BiExpression<'a>),
    SubPow(TriExpression<'a>),
    Group(Group<'a>),
    Unary(Unary<'a>),
    Binary(Binary<'a>),
    Literal(Literal<'a>),
    Expressions(Expressions<'a>),
    Unit,
}

#[derive(Debug, Clone)]
pub struct TriExpression<'a> {
    pub expression_1: Box<Expression<'a>>,
    pub expression_2: Box<Expression<'a>>,
    pub expression_3: Box<Expression<'a>>,
}

#[derive(Debug, Clone)]
pub struct BiExpression<'a> {
    pub expression_1: Box<Expression<'a>>,
    pub expression_2: Box<Expression<'a>>,
}

#[derive(Debug, Clone)]
pub struct Unary<'a> {
    pub operator: Token<'a>,
    pub expression: Box<Expression<'a>>,
}

#[derive(Debug, Clone)]
pub struct Binary<'a> {
    pub operator: Token<'a>,
    pub expression_1: Box<Expression<'a>>,
    pub expression_2: Box<Expression<'a>>,
}

#[derive(Debug, Clone)]
pub struct Group<'a> {
    pub l_brace: Token<'a>,
    pub expressions: Expressions<'a>,
    pub r_brace: Token<'a>,
}

#[derive(Debug, Clone)]
pub enum Literal<'a> {
    Literal(Token<'a>), // borrow cst literals
    Table(Table<'a>),
}

#[derive(Debug, Clone)]
pub struct Table<'a> {
    pub seperators: Vec<usize>,
    pub l_brace: Token<'a>,
    // first vec seperates tables that need seperation by a seperator
    // second vec seperates rows
    // third vec seperates columns
    pub rows: Vec<TableRow<'a>>,
    pub r_brace: Token<'a>,
}

#[derive(Debug, Clone)]
pub struct TableRow<'a> {
    pub cols: Vec<Expressions<'a>>,
}
