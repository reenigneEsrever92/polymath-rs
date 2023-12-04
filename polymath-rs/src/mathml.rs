use itertools::Itertools;

use crate::{
    ast::{Expression, Expressions, Literal, Table, TableRow, AST},
    tokens::{
        types::{
            Arrow, BinaryOperator, Function, Greek, LBrace, Logical, Misc, Operation, RBrace,
            Relational, TokenType, UnaryOperator,
        },
        Token,
    },
};

pub fn to_mathml(ast: &AST) -> String {
    format!(
        "<math display=\"block\">{}</math>",
        expressions_to_mathml(&ast.expressions)
    )
}

fn expressions_to_mathml(expressions: &Expressions) -> String {
    expressions
        .expressions
        .iter()
        .map(|expression| expression_to_mathml(expression))
        .join("")
}

fn expression_to_mathml(expr: &Expression) -> String {
    match expr {
        Expression::Frac(frac) => frac_to_mathml(frac),
        Expression::Sub(bi_expression) => sub_to_mathml(bi_expression),
        Expression::Pow(bi_expression) => pow_to_mathml(bi_expression),
        Expression::SubPow(tri_expression) => sub_pow_to_mathml(tri_expression),
        Expression::Group(group) => group_to_mathml(group),
        Expression::Unary(unary) => unary_to_mathml(unary),
        Expression::Binary(binary) => binary_to_mathml(binary),
        Expression::Literal(literal) => literal_to_mathml(literal),
        Expression::Expressions(expressions) => expressions_to_mathml(expressions),
        Expression::Unit => unreachable!(),
    }
}

fn binary_to_mathml(binary: &crate::ast::Binary) -> String {
    match binary.operator.token_type {
        crate::tokens::types::TokenType::BinaryOperator(BinaryOperator::Root) => format!(
            "<mroot><mrow>{}</mrow><mrow>{}</mrow></mroot>",
            match binary.expression_2.as_ref() {
                Expression::Group(group) => expressions_to_mathml(&group.expressions),
                _ => expression_to_mathml(&binary.expression_2),
            },
            match binary.expression_1.as_ref() {
                Expression::Group(group) => expressions_to_mathml(&group.expressions),
                _ => expression_to_mathml(&binary.expression_1),
            }
        ),
        crate::tokens::types::TokenType::BinaryOperator(BinaryOperator::Overset) => format!(
            "<mover><mrow>{}</mrow><mrow>{}</mrow></mover>",
            match binary.expression_2.as_ref() {
                Expression::Group(group) => expressions_to_mathml(&group.expressions),
                _ => expression_to_mathml(&binary.expression_2),
            },
            match binary.expression_1.as_ref() {
                Expression::Group(group) => expressions_to_mathml(&group.expressions),
                _ => expression_to_mathml(&binary.expression_1),
            }
        ),
        crate::tokens::types::TokenType::BinaryOperator(BinaryOperator::Underset) => format!(
            "<munder><mrow>{}</mrow><mrow>{}</mrow></munder>",
            match binary.expression_2.as_ref() {
                Expression::Group(group) => expressions_to_mathml(&group.expressions),
                _ => expression_to_mathml(&binary.expression_2),
            },
            match binary.expression_1.as_ref() {
                Expression::Group(group) => expressions_to_mathml(&group.expressions),
                _ => expression_to_mathml(&binary.expression_1),
            }
        ),
        crate::tokens::types::TokenType::BinaryOperator(BinaryOperator::Color) => format!(
            "<mstyle mathcolor=\"{}\">{}</mstyle>",
            match binary.expression_1.as_ref() {
                Expression::Group(group) => group
                    .expressions
                    .expressions
                    .iter()
                    .map(|expression| match expression {
                        Expression::Literal(Literal::Literal(literal)) => literal.span.text,
                        _ => "",
                    })
                    .join(""),
                _ => "".to_string(),
            },
            match binary.expression_2.as_ref() {
                Expression::Group(group) => expressions_to_mathml(&group.expressions),
                _ => expression_to_mathml(&binary.expression_2),
            },
        ),
        _ => format!(
            "{}{}{}",
            binary.operator.span.text,
            expression_to_mathml(&binary.expression_1),
            expression_to_mathml(&binary.expression_2),
        ),
    }
}

fn group_to_mathml(group: &crate::ast::Group) -> String {
    format!(
        "<mrow>{}{}{}</mrow>",
        l_brace_to_math_ml(&group.l_brace),
        group
            .expressions
            .expressions
            .iter()
            .map(|expr| expression_to_mathml(expr))
            .join(""),
        r_brace_to_math_ml(&group.r_brace),
    )
}

fn sub_pow_to_mathml(tri_expression: &crate::ast::TriExpression) -> String {
    if let Expression::Literal(Literal::Literal(literal)) = tri_expression.expression_1.as_ref() {
        match &literal.token_type {
            TokenType::Operation(op) => match op {
                Operation::Sum
                | Operation::Prod
                | Operation::BigWedge
                | Operation::BigCap
                | Operation::BigCup => {
                    return format!(
                        "<munderover>{}{}{}</munderover>",
                        expression_to_mathml_braceless(&tri_expression.expression_1),
                        expression_to_mathml_braceless(&tri_expression.expression_2),
                        expression_to_mathml_braceless(&tri_expression.expression_3)
                    )
                }
                _ => {}
            },
            TokenType::Misc(Misc::Lim) => {
                return format!(
                    "<munderover>{}{}{}</munderover>",
                    expression_to_mathml_braceless(&tri_expression.expression_1),
                    expression_to_mathml_braceless(&tri_expression.expression_2),
                    expression_to_mathml_braceless(&tri_expression.expression_3)
                )
            }
            _ => {}
        }
    } else if let Expression::Unary(unary) = tri_expression.expression_1.as_ref() {
        match unary.operator.token_type {
            TokenType::UnaryOperator(UnaryOperator::UBrace) => {
                return format!(
                    "<munderover><munder>{}</munder>{}{}</munderover>",
                    expression_to_mathml_braceless(&tri_expression.expression_1),
                    expression_to_mathml_braceless(&tri_expression.expression_2),
                    expression_to_mathml_braceless(&tri_expression.expression_3)
                )
            }
            TokenType::UnaryOperator(UnaryOperator::OBrace) => {
                return format!(
                    "<munderover><mover>{}</mover>{}{}</munderover>",
                    expression_to_mathml_braceless(&tri_expression.expression_1),
                    expression_to_mathml_braceless(&tri_expression.expression_2),
                    expression_to_mathml_braceless(&tri_expression.expression_3)
                )
            }
            _ => {}
        }
    }
    format!(
        "<msubsup>{}{}{}</msubsup>",
        expression_to_mathml(&tri_expression.expression_1),
        expression_to_mathml(&tri_expression.expression_2),
        expression_to_mathml(&tri_expression.expression_3)
    )
}

fn pow_to_mathml(bi_expression: &crate::ast::BiExpression) -> String {
    if let Expression::Literal(Literal::Literal(token)) = bi_expression.expression_1.as_ref() {
        match &token.token_type {
            TokenType::Operation(op) => match op {
                Operation::Sum
                | Operation::Prod
                | Operation::BigWedge
                | Operation::BigCap
                | Operation::BigCup => {
                    return format!(
                        "<mover>{}{}</mover>",
                        expression_to_mathml_braceless(&bi_expression.expression_1),
                        expression_to_mathml_braceless(&bi_expression.expression_2)
                    )
                }
                _ => {}
            },
            TokenType::Misc(Misc::Lim) => {
                return format!(
                    "<mover>{}{}</mover>",
                    expression_to_mathml_braceless(&bi_expression.expression_1),
                    expression_to_mathml_braceless(&bi_expression.expression_2)
                )
            }
            _ => {}
        }
    } else if let Expression::Unary(unary) = bi_expression.expression_1.as_ref() {
        if let TokenType::UnaryOperator(UnaryOperator::OBrace) = unary.operator.token_type {
            return format!(
                "<mover><mover>{}</mover>{}</mover>",
                expression_to_mathml_braceless(&bi_expression.expression_1),
                expression_to_mathml_braceless(&bi_expression.expression_2),
            );
        }
    }

    format!(
        "<msup>{}{}</msup>",
        expression_to_mathml(&bi_expression.expression_1),
        expression_to_mathml(&bi_expression.expression_2)
    )
}

fn sub_to_mathml(bi_expression: &crate::ast::BiExpression) -> String {
    if let Expression::Literal(Literal::Literal(literal)) = bi_expression.expression_1.as_ref() {
        match &literal.token_type {
            TokenType::Operation(op) => match op {
                Operation::Sum
                | Operation::Prod
                | Operation::BigWedge
                | Operation::BigCap
                | Operation::BigCup => {
                    return format!(
                        "<munder>{}{}</munder>",
                        expression_to_mathml_braceless(&bi_expression.expression_1),
                        expression_to_mathml_braceless(&bi_expression.expression_2)
                    )
                }
                _ => {}
            },
            TokenType::Misc(Misc::Lim) => {
                return format!(
                    "<munder>{}{}</munder>",
                    expression_to_mathml_braceless(&bi_expression.expression_1),
                    expression_to_mathml_braceless(&bi_expression.expression_2)
                )
            }
            _ => {}
        }
    } else if let Expression::Unary(unary) = bi_expression.expression_1.as_ref() {
        if let TokenType::UnaryOperator(UnaryOperator::UBrace) = unary.operator.token_type {
            return format!(
                "<munder><munder>{}</munder>{}</munder>",
                expression_to_mathml_braceless(&bi_expression.expression_1),
                expression_to_mathml_braceless(&bi_expression.expression_2),
            );
        }
    }

    format!(
        "<msub>{}{}</msub>",
        expression_to_mathml(&bi_expression.expression_1),
        expression_to_mathml(&bi_expression.expression_2)
    )
}

fn expression_to_mathml_braceless(expression: &Expression) -> String {
    match expression {
        Expression::Group(group) => {
            format!("<mrow>{}</mrow>", expressions_to_mathml(&group.expressions))
        }
        _ => expression_to_mathml(expression),
    }
}

fn frac_to_mathml(frac: &crate::ast::BiExpression) -> String {
    format!(
        "<mfrac>{}{}</mfrac>",
        expression_to_mathml_braceless(&frac.expression_1),
        expression_to_mathml_braceless(&frac.expression_2)
    )
}

fn unary_to_mathml(unary: &crate::ast::Unary) -> String {
    match unary.operator.token_type {
        TokenType::UnaryOperator(UnaryOperator::Hat)
        | TokenType::UnaryOperator(UnaryOperator::Bar)
        | TokenType::UnaryOperator(UnaryOperator::Vec)
        | TokenType::UnaryOperator(UnaryOperator::Tilde)
        | TokenType::UnaryOperator(UnaryOperator::Dot)
        | TokenType::UnaryOperator(UnaryOperator::DDot)
        | TokenType::UnaryOperator(UnaryOperator::OBrace) => {
            let symbol = match unary.operator.token_type {
                TokenType::UnaryOperator(UnaryOperator::Hat) => "^",
                TokenType::UnaryOperator(UnaryOperator::Bar) => "&#xAF;",
                TokenType::UnaryOperator(UnaryOperator::Vec) => "&#x2192;",
                TokenType::UnaryOperator(UnaryOperator::Tilde) => "~",
                TokenType::UnaryOperator(UnaryOperator::Dot) => ".",
                TokenType::UnaryOperator(UnaryOperator::DDot) => "..",
                TokenType::UnaryOperator(UnaryOperator::OBrace) => "&#x23DE;",
                _ => "",
            };

            format!(
                "<mover>{}<mo>{}</mo></mover>",
                expression_to_mathml_braceless(&unary.expression),
                symbol
            )
        }
        TokenType::UnaryOperator(UnaryOperator::Ul)
        | TokenType::UnaryOperator(UnaryOperator::UBrace) => {
            let symbol = match unary.operator.token_type {
                TokenType::UnaryOperator(UnaryOperator::Ul) => "&#x332;",
                TokenType::UnaryOperator(UnaryOperator::UBrace) => "&#x23DF;",
                _ => "",
            };

            format!(
                "<munder>{}<mo>{}</mo></munder>",
                expression_to_mathml_braceless(&unary.expression),
                symbol
            )
        }
        TokenType::UnaryOperator(UnaryOperator::Abs)
        | TokenType::UnaryOperator(UnaryOperator::Floor)
        | TokenType::UnaryOperator(UnaryOperator::Ceil)
        | TokenType::UnaryOperator(UnaryOperator::Norm) => {
            let (left_symbol, right_symbol) = match unary.operator.token_type {
                TokenType::UnaryOperator(UnaryOperator::Abs) => ("|", "|"),
                TokenType::UnaryOperator(UnaryOperator::Floor) => ("&#x230A;", "&#x230B;"),
                TokenType::UnaryOperator(UnaryOperator::Ceil) => ("&#x2308;", "&#x2309;"),
                TokenType::UnaryOperator(UnaryOperator::Norm) => ("&#x2225;", "&#x2225;"),
                _ => ("", ""),
            };

            format!(
                "<mo>{}</mo>{}<mo>{}</mo>",
                left_symbol,
                expression_to_mathml_braceless(&unary.expression),
                right_symbol
            )
        }
        TokenType::UnaryOperator(UnaryOperator::Cancel) => format!(
            "<menclose notation=\"updiagonalstrike\">{}</menclose>",
            expression_to_mathml_braceless(&unary.expression)
        ),
        TokenType::UnaryOperator(UnaryOperator::Sqrt) => {
            format!(
                "<msqrt>{}</msqrt>",
                expression_to_mathml_braceless(&unary.expression)
            )
        }
        TokenType::UnaryOperator(UnaryOperator::Text) => {
            format!(
                "<mtext>{}</mtext>",
                expression_to_mathml_braceless(&unary.expression)
            )
        }
        _ => "".to_string(),
    }
}

fn literal_to_mathml(literal: &crate::ast::Literal) -> String {
    match literal {
        Literal::Literal(token) => token_to_mathml(token),
        Literal::Table(table) => table_to_mathml(table),
    }
}

fn table_to_mathml(table: &crate::ast::Table) -> String {
    format!(
        "<mrow>{}<mtable {}>{}</mtable>{}</mrow>",
        l_brace_to_math_ml(&table.l_brace),
        format_column_line(table),
        expressions_to_mathml_table(&table.rows),
        r_brace_to_math_ml(&table.r_brace)
    )
}

fn format_column_line(table: &Table) -> String {
    table
        .rows
        .first()
        .map(|row| {
            format!(
                "columnlines=\"{}\"",
                row.cols
                    .iter()
                    .enumerate()
                    .skip(1)
                    .map(|(index, _)| {
                        if table.seperators.contains(&index) {
                            "solid".to_string()
                        } else {
                            "none".to_string()
                        }
                    })
                    .join(" ")
            )
        })
        .unwrap_or("".to_string())
}

fn expressions_to_mathml_table(rows: &[TableRow]) -> String {
    rows.iter()
        .map(|row| {
            format!(
                "<mtr>{}</mtr>",
                row.cols
                    .iter()
                    .map(|col| format!("<mtd>{}</mtd>", expressions_to_mathml(col)))
                    .join("")
            )
        })
        .join("")
}

fn l_brace_to_math_ml(token: &Token) -> String {
    match &token.token_type {
        TokenType::LBrace(lbrace) => match lbrace {
            LBrace::LParen => "<mo>(</mo>".to_string(),
            LBrace::LBracket => "<mo>[</mo>".to_string(),
            LBrace::LBrace => "<mo>{</mo>".to_string(),
            LBrace::LColonBrace => "".to_string(),
            LBrace::LAngle => "<mo><</mo>".to_string(),
        },
        _ => token.span.text.to_string(),
    }
}

fn r_brace_to_math_ml(token: &Token) -> String {
    match &token.token_type {
        TokenType::RBrace(rbrace) => match rbrace {
            RBrace::RParen => "<mo>)</mo>".to_string(),
            RBrace::RBracket => "<mo>]</mo>".to_string(),
            RBrace::RBrace => "<mo>}</mo>".to_string(),
            RBrace::RColonBrace => "".to_string(),
            RBrace::RAngle => "<mo>></mo>".to_string(),
        },
        _ => token.span.text.to_string(),
    }
}

fn token_to_mathml(token: &Token) -> String {
    match &token.token_type {
        TokenType::Symbol => format!("<mi>{}</mi>", token.span.text),
        TokenType::Greek(greek) => greek_to_mathml(greek),
        TokenType::Operation(op) => format!("<mo>{}</mo>", operation_to_mathml(op)),
        TokenType::Misc(misc) => misc_to_mathml(misc),
        TokenType::Relational(relational) => relational_to_mathml(relational),
        TokenType::Arrow(arrow) => format!("<mo>{}</mo>", arrow_to_mathml(arrow)),
        TokenType::Logical(logical) => logical_to_mathml(logical),
        TokenType::Number => format!("<mn>{}</mn>", token.span.text),
        TokenType::Text => format!("<mtext>{}</mtext>", token.span.text),
        TokenType::Function(function) => format!("<mi>{}</mi>", function_to_mathml(function)),
        TokenType::None => "".to_string(),
        _ => format!("<mi>{}</mi>", token.span.text),
    }
}

fn greek_to_mathml(greek: &Greek) -> String {
    let symbol = match greek {
        Greek::Alpha => "&#x3B1;",
        Greek::Beta => "&#x3B2;",
        Greek::Gamma => "&#x3B2;",
        Greek::UGamma => "&#x393;",
        Greek::Delta => "&#x3B4;",
        Greek::UDelta => "&#x394;",
        Greek::Epsilon => "&#x3B5;",
        Greek::VarEpsilon => "&#x25B;",
        Greek::Zeta => "&#x3B6;",
        Greek::Eta => "&#x3B7;",
        Greek::Theta => "&#x3B8;",
        Greek::UTheta => "&#x398;",
        Greek::VarTheta => "&#x3D1;",
        Greek::Iota => "&#x3B9;",
        Greek::Kappa => "&#x3BA;",
        Greek::Lambda => "&#x3BB;",
        Greek::ULambda => "&#x39B;",
        Greek::Mu => "&#x3BC;",
        Greek::Nu => "&#x3BD;",
        Greek::Xi => "&#x3BE;",
        Greek::UXi => "&#x39E;",
        Greek::Pi => "&#x3C0;",
        Greek::UPi => "&#x3A0;",
        Greek::Rho => "&#x3C1;",
        Greek::Sigma => "&#x3C3;",
        Greek::USigma => "&#x3A3;",
        Greek::Tau => "&#x3C4;",
        Greek::Upsilon => "&#x3C5;",
        Greek::Phi => "&#x3D5;",
        Greek::UPhi => "&#x3A6;",
        Greek::VarPhi => "&#x3C6;",
        Greek::Chi => "&#x3C7;",
        Greek::Psi => "&#x3C8;",
        Greek::UPsi => "&#x3A8;",
        Greek::Omega => "&#x3C9;",
        Greek::UOmega => "&#x3A9;",
    };

    format!("<mi>{symbol}</mi>")
}

fn logical_to_mathml(logical: &Logical) -> String {
    match logical {
        Logical::Not
        | Logical::Implies
        | Logical::Iff
        | Logical::ForAll
        | Logical::Exists
        | Logical::Bot
        | Logical::Top
        | Logical::VDash
        | Logical::Models => {
            let symbol = match logical {
                Logical::Not => "&#xAC;",
                Logical::Implies => "&#x21D2;",
                Logical::Iff => "&#x21D4;",
                Logical::ForAll => "&#x2200;",
                Logical::Exists => "&#x2203;",
                Logical::Bot => "&#x22A5;",
                Logical::Top => "&#x22A4;",
                Logical::VDash => "&#x22A2;",
                Logical::Models => "&#x22A8;",
                _ => "",
            };

            format!("<mo>{symbol}</mo>")
        }
        Logical::And | Logical::Or | Logical::If => {
            let symbol = match logical {
                Logical::And => "and",
                Logical::Or => "or",
                Logical::If => "if",
                _ => "",
            };

            format!("<mrow><mspace width=\"1ex\" /><mtext>{symbol}</mtext><msapce with=\"1ex\" /></mrow>")
        }
    }
}

fn function_to_mathml(function: &Function) -> &'static str {
    match function {
        Function::Sin => "sin",
        Function::Cos => "cos",
        Function::Tan => "tan",
        Function::Sec => "sec",
        Function::Csc => "csc",
        Function::Cot => "cot",
        Function::Arcsin => "arcsin",
        Function::Arccos => "arccos",
        Function::Arctan => "arctan",
        Function::Sinh => "sinh",
        Function::Cosh => "cosh",
        Function::Tanh => "tanh",
        Function::Sech => "sech",
        Function::Csch => "csch",
        Function::Coth => "coth",
        Function::Exp => "exp",
        Function::Log => "log",
        Function::Ln => "ln",
        Function::Det => "det",
        Function::Dim => "dim",
        Function::Mod => "mod",
        Function::Gcd => "gcd",
        Function::Lcm => "lcm",
        Function::Lub => "lub",
        Function::Glb => "glb",
        Function::Min => "min",
        Function::Max => "max",
        Function::F => "f",
        Function::G => "g",
    }
}

fn arrow_to_mathml(arrow: &Arrow) -> &'static str {
    match arrow {
        Arrow::UpArrow => "&#x2191;",
        Arrow::DownArrow => "&#x2193;",
        Arrow::RightArrow => "&#x2192;",
        Arrow::ToArrow => "&#x2192;",
        Arrow::RightArrowTail => "&#x21A3;",
        Arrow::RightArrowTwoHead => "&#x21A0;",
        Arrow::RightArrowTwoHeadTail => "&#x2916;",
        Arrow::MapsTo => "&#x21A6;",
        Arrow::LeftArrow => "&#x2190;",
        Arrow::LeftRightArrow => "&#x2194;",
        Arrow::DoubleRightArrow => "&#x21D2;",
        Arrow::DoubleLeftArrow => "&#x21D0;",
        Arrow::DoubleLeftRightArrow => "&#x21D4;",
    }
}

fn relational_to_mathml(relational: &Relational) -> String {
    match relational {
        Relational::Equals
        | Relational::NotEquals
        | Relational::Lt
        | Relational::Gt
        | Relational::Lte
        | Relational::Gte
        | Relational::Prec
        | Relational::PrecEq
        | Relational::Succ
        | Relational::SuccEq
        | Relational::In
        | Relational::NotIn
        | Relational::Sub
        | Relational::Sup
        | Relational::SubEq
        | Relational::SupEq
        | Relational::Equiv
        | Relational::Cong
        | Relational::Approx
        | Relational::Prop => {
            let symbol = match relational {
                Relational::Equals => "=",
                Relational::NotEquals => "&#x2260;",
                Relational::Lt => "&lt;",
                Relational::Gt => "&gt;",
                Relational::Lte => "&#x2264;",
                Relational::Gte => "&#x2265;",
                Relational::Prec => "&#x227A;",
                Relational::PrecEq => "&#x2AAF;",
                Relational::Succ => "&#x227B;",
                Relational::SuccEq => "&#x2AB0;",
                Relational::In => "&#x2208;",
                Relational::NotIn => "&#x2209;",
                Relational::Sub => "&#x2282;",
                Relational::SubEq => "&#x2286;",
                Relational::Sup => "&#x2283;",
                Relational::SupEq => "&#x2287;",
                Relational::Equiv => "&#x2261;",
                Relational::Cong => "&#x2245;",
                Relational::Approx => "&#x2248;",
                Relational::Prop => "&#x221D;",
                _ => "",
            };

            format!("<mo>{}</mo>", symbol)
        }
        Relational::Mlt => "<mi>m</mi><mo>&lt;</mo>".to_string(),
        Relational::Mgt => "<mi>m</mi><mo>&gt;</mo>".to_string(),
    }
}

fn misc_to_mathml(misc: &Misc) -> String {
    match misc {
        Misc::DoublePipes => {
            return "<mrow><mo>&#x2223;</mo></mrow><mrow><mo>&#x2223;</mo></mrow>".to_string()
        }
        Misc::DoublePipesQuad => {
            return "<mrow><mo>|</mo><mo>&#xA0;&#xA0;</mo><mo>|</mo></mrow>".to_string()
        }
        _ => {}
    }

    let symbol = match misc {
        Misc::Int => "&#x222B;",
        Misc::OInt => "&#x222E;",
        Misc::Del => "&#x2202;",
        Misc::Grad => "&#x2207;",
        Misc::PlusMinus => "&#xB1;",
        Misc::EmptySet => "&#x2205;",
        Misc::Infinity => "&#x221E;",
        Misc::Aleph => "&#x2135;",
        Misc::Therefore => "&#x2234;",
        Misc::Because => "&#x2235;",
        Misc::LDots => "...",
        Misc::CDots => "&#x22EF;",
        Misc::VDots => "&#x22EE;",
        Misc::DDots => "&#x22F1;",
        Misc::Angle => "&#x2220;",
        Misc::Frown => "&#x2322;",
        Misc::Triangle => "&#x25B3;",
        Misc::Diamond => "&#x22C4;",
        Misc::Square => "&#x25A1;",
        Misc::LFloor => "&#x230A;",
        Misc::RFloor => "&#x230B;",
        Misc::LCeiling => "&#x2308;",
        Misc::RCeiling => "&#x2309;",
        Misc::CC => "&#x2102;",
        Misc::NN => "&#x2115;",
        Misc::QQ => "&#x211A;",
        Misc::RR => "&#x211D;",
        Misc::ZZ => "&#x2124;",
        Misc::DoublePipes => "",
        Misc::DoublePipesQuad => "",
        Misc::Lim => "lim",
    };

    format!("<mo>{symbol}</mo>")
}

fn operation_to_mathml(op: &Operation) -> &'static str {
    match op {
        Operation::Plus => "+",
        Operation::Minus => "-",
        Operation::CDot => "&#x22C5;",
        Operation::Ast => "&#x2217;",
        Operation::Star => "&#x22C6;",
        Operation::Slash => "/",
        Operation::Backslash => "\\",
        Operation::Times => "&#xD7;",
        Operation::Div => "&#xF7;",
        Operation::LTimes => "&#x22C9;",
        Operation::RTimes => "&#x22CA;",
        Operation::Bowtie => "&#x22C8;",
        Operation::Circ => "&#x2218;",
        Operation::OPlus => "&#x2295;",
        Operation::OTimes => "&#x2297;",
        Operation::ODot => "&#x2299;",
        Operation::Sum => "&#x2211;",
        Operation::Prod => "&#x220F;",
        Operation::Wedge => "&#x2227;",
        Operation::BigWedge => "&#x22C0;",
        Operation::Vee => "&#x2228;",
        Operation::BigVee => "&#x22C1;",
        Operation::Cap => "&#x2229;",
        Operation::BigCap => "&#x22C2;",
        Operation::Cup => "&#x222A;",
        Operation::BigCup => "&#x22C3;",
    }
}
