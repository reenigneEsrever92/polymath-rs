//!
//! This module is used to convert the PredictiveCST into an AST
//!

use std::collections::VecDeque;

use crate::{
    cst::predictive::{
        Expression as CSTExpression, IntermediateExpression, PredictiveCST, SimpleExpression,
    },
    tokens::Token,
};

use super::{
    BiExpression, Binary, Expression, Expressions, Group, Literal, TriExpression, Unary, AST,
};

pub fn convert<'a>(cst: &'a PredictiveCST<'a>) -> AST<'a> {
    AST {
        expressions: convert_e(&cst.expression),
    }
}

fn convert_e<'a>(expression: &CSTExpression<'a>) -> Expressions<'a> {
    let mut expressions = VecDeque::new();

    match &expression {
        CSTExpression::IE(left, right) => {
            expressions.append(&mut convert_ie(left, right).expressions);
        }
        CSTExpression::II(left, _, right) => {
            let mut right_expressions = convert_e(right);

            expressions.push_back(Expression::Frac(BiExpression {
                expression_1: Box::new(convert_i(left)),
                expression_2: Box::new(right_expressions.expressions.pop_front().unwrap()),
            }));

            expressions.append(&mut right_expressions.expressions);
        }
        CSTExpression::Unit => {}
    };

    Expressions { expressions }
}

fn convert_ie<'a>(left: &IntermediateExpression<'a>, right: &CSTExpression<'a>) -> Expressions<'a> {
    let left = convert_i(left);
    let mut expressions = VecDeque::new();
    let mut right = convert_e(right);

    expressions.push_back(left);
    expressions.append(&mut right.expressions);

    Expressions { expressions }
}

fn convert_i<'a>(expr: &IntermediateExpression<'a>) -> Expression<'a> {
    match expr {
        IntermediateExpression::SimpleSub(s1, _, s2) => Expression::Sub(BiExpression {
            expression_1: Box::new(convert_simple(s1)),
            expression_2: Box::new(convert_simple(s2)),
        }),
        IntermediateExpression::SimpleSup(s1, _, s2) => Expression::Pow(BiExpression {
            expression_1: Box::new(convert_simple(s1)),
            expression_2: Box::new(convert_simple(s2)),
        }),
        IntermediateExpression::SimpleSubSup(s1, _, s2, _, s3) => {
            Expression::SubPow(TriExpression {
                expression_1: Box::new(convert_simple(s1)),
                expression_2: Box::new(convert_simple(s2)),
                expression_3: Box::new(convert_simple(s3)),
            })
        }
        IntermediateExpression::Simple(simple) => convert_simple(simple),
    }
}

fn convert_simple<'a>(expr: &SimpleExpression<'a>) -> Expression<'a> {
    match expr {
        SimpleExpression::Symbol(symbol) => Expression::Literal(convert_literal(symbol)),
        SimpleExpression::Group(l_brace, expr, r_brace) => Expression::Group(Group {
            l_brace: l_brace.to_owned(),
            expressions: convert_e(expr),
            r_brace: r_brace.to_owned(),
        }),
        SimpleExpression::UnarySymbol(operator, e) => Expression::Unary(Unary {
            operator: operator.clone(),
            expression: Box::new(convert_simple(e.as_ref())),
        }),
        SimpleExpression::BinarySymbol(operator, e, e1) => Expression::Binary(Binary {
            operator: operator.clone(),
            expression_1: Box::new(convert_simple(e)),
            expression_2: Box::new(convert_simple(e1)),
        }),
    }
}

fn convert_literal<'a>(literal: &Token<'a>) -> Literal<'a> {
    Literal::Literal(literal.to_owned())
}
