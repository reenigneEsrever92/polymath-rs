use std::collections::VecDeque;

use crate::{
    ast::{
        BiExpression, Binary, Expression, Expressions, Group, Literal, Table, TableRow,
        TriExpression, Unary, AST,
    },
    tokens::types::TokenType,
};

pub fn transform(ast: AST) -> AST {
    // TODO apply matrix / vector transformations here
    AST {
        expressions: transform_expressions(ast.expressions),
    }
}

fn transform_expressions(expressions: Expressions) -> Expressions {
    Expressions {
        expressions: expressions
            .expressions
            .into_iter()
            .map(|expr| transform_expression(expr))
            .collect(),
    }
}

fn transform_expression(expr: Expression) -> Expression {
    match expr {
        Expression::Group(group) => transform_group(Group {
            l_brace: group.l_brace,
            expressions: transform_expressions(group.expressions),
            r_brace: group.r_brace,
        }),
        Expression::Frac(frac) => transform_frac(frac),
        Expression::Sub(bi_expression) => Expression::Sub(transform_bi_expression(bi_expression)),
        Expression::Pow(bi_expression) => Expression::Pow(transform_bi_expression(bi_expression)),
        Expression::SubPow(tri_expression) => {
            Expression::SubPow(transform_tri_expression(tri_expression))
        }
        Expression::Unary(unary) => Expression::Unary(transform_unary(unary)),
        Expression::Binary(binary) => Expression::Binary(transform_binary(binary)),
        Expression::Literal(lit) => Expression::Literal(lit),
        Expression::Expressions(expressions) => {
            Expression::Expressions(transform_expressions(expressions))
        }
        Expression::Unit => Expression::Unit,
    }
}

fn transform_binary(binary: crate::ast::Binary) -> crate::ast::Binary {
    Binary {
        operator: binary.operator,
        expression_1: Box::new(transform_expression(*binary.expression_1)),
        expression_2: Box::new(transform_expression(*binary.expression_2)),
    }
}

fn transform_tri_expression(
    tri_expression: crate::ast::TriExpression,
) -> crate::ast::TriExpression {
    TriExpression {
        expression_1: Box::new(transform_expression(*tri_expression.expression_1)),
        expression_2: Box::new(transform_expression(*tri_expression.expression_2)),
        expression_3: Box::new(transform_expression(*tri_expression.expression_3)),
    }
}

fn transform_bi_expression(bi_expression: BiExpression) -> BiExpression {
    BiExpression {
        expression_1: Box::new(transform_expression(*bi_expression.expression_1)),
        expression_2: Box::new(transform_expression(*bi_expression.expression_2)),
    }
}

fn transform_frac(frac: crate::ast::BiExpression) -> Expression {
    Expression::Frac(transform_bi_expression(frac))
}

fn transform_unary(unary: Unary) -> Unary {
    Unary {
        operator: unary.operator,
        expression: Box::new(transform_expression(*unary.expression)),
    }
}

fn transform_group<'a>(group: Group<'a>) -> Expression<'a> {
    let group = Group {
        l_brace: group.l_brace,
        expressions: transform_expressions(group.expressions),
        r_brace: group.r_brace,
    };

    let expressions = &group.expressions.expressions;

    let group_expressions = expressions
        .iter()
        .enumerate()
        .filter_map(|(index, expr)| if index % 2 == 0 { Some(expr) } else { None })
        .collect::<Vec<&Expression<'a>>>();

    let commas = expressions
        .iter()
        .skip(1)
        .enumerate()
        .filter_map(|(index, expr)| if index % 2 == 0 { Some(expr) } else { None })
        .collect::<Vec<&Expression<'a>>>();

    let all_groups = group_expressions
        .iter()
        .all(|expr| matches!(expr, Expression::Group(_)));

    let all_commas = commas.iter().all(|comma| match comma {
        Expression::Literal(lit) => {
            matches!(lit, crate::ast::Literal::Literal(token) if match token.token_type {
                TokenType::Symbol => matches!(token.span.text, ","),
                _ => false,
            })
        }
        _ => false,
    });

    if !all_groups || !all_commas {
        return Expression::Group(group);
    }

    let groups = group_expressions
        .iter()
        .map(|expression| match expression {
            Expression::Group(group) => group,
            _ => unreachable!(),
        })
        .collect::<Vec<&Group<'a>>>();

    let group_comma_indicies = groups
        .iter()
        .map(|group| {
            group
                .expressions
                .expressions
                .iter()
                .enumerate()
                .filter_map(|(index, expr)| match expr {
                    Expression::Literal(lit) => {
                        if let crate::ast::Literal::Literal(token) = lit {
                            if let TokenType::Symbol = token.token_type {
                                if let "," = token.span.text {
                                    Some(index)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let comma_counts_match = group_comma_indicies
        .iter()
        .all(|commas| commas.len() == group_comma_indicies[0].len());

    if !comma_counts_match {
        return Expression::Group(group);
    }

    let mut table_contents = groups
        .iter()
        .zip(&group_comma_indicies)
        .map(|(group, group_commas)| {
            let mut group_ranges =
                group_commas
                    .iter()
                    .fold(Vec::<(usize, usize)>::new(), |mut vec, comma_pos| {
                        if let Some((_, last_pos)) = vec.last() {
                            vec.push((*last_pos + 1, *comma_pos));
                        } else {
                            vec.push((0, *comma_pos));
                        }
                        vec
                    });

            group_ranges.push((
                group_commas.last().map(|index| index + 1).unwrap_or(0),
                group.expressions.expressions.len(),
            ));

            TableRow {
                cols: group_ranges
                    .iter()
                    .map(|(start_pos, end_pos)| Expressions {
                        expressions: group
                            .expressions
                            .expressions
                            .iter()
                            .skip(*start_pos)
                            .take(end_pos - start_pos)
                            .cloned()
                            .collect::<VecDeque<Expression<'a>>>(),
                    })
                    .collect::<Vec<Expressions<'a>>>(),
            }
        })
        .collect::<Vec<TableRow>>();

    let seperator_pos = table_contents
        .first()
        .iter()
        .flat_map(|row| {
            row.cols
                .iter()
                .enumerate()
                .filter(|(_, col)| {
                    col.expressions.iter().any(|expr| match expr {
                        Expression::Literal(Literal::Literal(token)) => {
                            matches!(token.span.text, "|")
                        }
                        _ => false,
                    })
                })
                .map(|(index, _)| index)
        })
        .collect::<Vec<usize>>();

    // remove all vertical bars
    table_contents = table_contents
        .into_iter()
        .map(|row| TableRow {
            cols: row
                .cols
                .into_iter()
                .filter(|col| {
                    !col.expressions.iter().any(|expr| {
                        if let Expression::Literal(Literal::Literal(token)) = expr {
                            matches!(token.span.text, "|")
                        } else {
                            false
                        }
                    })
                })
                .collect::<Vec<Expressions<'a>>>(),
        })
        .collect::<Vec<TableRow>>();

    // fill first row back up again
    if let Some(second_row) = table_contents.get(1).map(|row| row.cols.len()) {
        if let Some(first_row) = table_contents.get_mut(0) {
            for i in 0..second_row {
                if first_row.cols.get(i).is_none() {
                    first_row.cols.push(Expressions {
                        expressions: VecDeque::new(),
                    });
                }
            }
        }
    }

    // TODO some combinations of parenthesis are actually not accepted for a table layout in the original asciimath
    Expression::Literal(crate::ast::Literal::Table(Table {
        seperators: seperator_pos,
        l_brace: group.l_brace,
        rows: table_contents,
        r_brace: group.r_brace,
    }))
}
