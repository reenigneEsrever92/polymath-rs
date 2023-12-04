use ast::predictive::convert;
use cst::predictive::parse;
use mathml::to_mathml;
use tracing::debug;
use transformations::transform;

use crate::tokens::tokenize;

pub mod ast;
pub mod cst;
pub mod mathml;
pub mod transformations;
pub mod tokens;

pub fn to_math_ml(content: &str) -> String {
    let tokens = tokenize(content);
    debug!("Tokens: {tokens:#?}");
    let cst = parse(&tokens);
    debug!("CST: {cst:#?}");
    let ast = convert(&cst);
    debug!("AST: {ast:#?}");
    let t_ast = transform(ast);
    debug!("AST after transformation: {t_ast:#?}");

    to_mathml(&t_ast)
}

#[cfg(test)]
mod test {
    // use tracing::Level;
    use crate::to_math_ml;

    macro_rules! test_parse {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                // tracing_subscriber::FmtSubscriber::builder().with_max_level(Level::DEBUG).init();
                let mathml = to_math_ml($input);
                assert_eq!(mathml, $expected);
            }
        };
    }

    test_parse!(
        test_parse1,
        "sum_b^a",
        "<math display=\"block\"><munderover><mo>&#x2211;</mo><mi>b</mi><mi>a</mi></munderover></math>"
    );

    test_parse!(
        test_frac,
        "(a+b)/6",
        "<math display=\"block\"><mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mn>6</mn></mfrac></math>"
    );

    test_parse!(
        test_vector,
        "[[1,2], [2, 23]]",
        "<math display=\"block\"><mrow><mo>[</mo><mtable columnlines=\"none\"><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd><mtd><mn>23</mn></mtd></mtr></mtable><mo>]</mo></mrow></math>"
    );

    test_parse!(
        test_vector_sum,
        "[[1],[sum_1^2a]]",
        "<math display=\"block\"><mrow><mo>[</mo><mtable columnlines=\"\"><mtr><mtd><mn>1</mn></mtd></mtr><mtr><mtd><munderover><mo>&#x2211;</mo><mn>1</mn><mn>2</mn></munderover><mi>a</mi></mtd></mtr></mtable><mo>]</mo></mrow></math>"
    );

    test_parse!(
        test_sum,
        "sum_1^2a",
        "<math display=\"block\"><munderover><mo>&#x2211;</mo><mn>1</mn><mn>2</mn></munderover><mi>a</mi></math>"
    );

    test_parse!(
        test_recusrsive_matrices, 
        "[[[[1],[2]]], [[[a], [b]]]]", 
        "<math display=\"block\"><mrow><mo>[</mo><mtable columnlines=\"\"><mtr><mtd><mrow><mo>[</mo><mtable columnlines=\"\"><mtr><mtd><mn>1</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable><mo>]</mo></mrow></mtd></mtr><mtr><mtd><mrow><mo>[</mo><mtable columnlines=\"\"><mtr><mtd><mi>a</mi></mtd></mtr><mtr><mtd><mi>b</mi></mtd></mtr></mtable><mo>]</mo></mrow></mtd></mtr></mtable><mo>]</mo></mrow></math>"
    );

    test_parse!(
        test_table_bar, 
        "[[1, |, 2], [a, b, c]]", 
        "<math display=\"block\"><mrow><mo>[</mo><mtable columnlines=\"solid none\"><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd><mtd></mtd></mtr><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd><mtd><mi>c</mi></mtd></mtr></mtable><mo>]</mo></mrow></math>"
    );

    test_parse!(
        test_table_complex,
        "{:[log_2 16 = 4,], [(2*5-3)/6, ]}",
        "<math display=\"block\"><mrow><mtable columnlines=\"none\"><mtr><mtd><msub><mi>log</mi><mn>2</mn></msub><mn>16</mn><mo>=</mo><mn>4</mn></mtd><mtd></mtd></mtr><mtr><mtd><mfrac><mrow><mn>2</mn><mo>&#x22C5;</mo><mn>5</mn><mo>-</mo><mn>3</mn></mrow><mn>6</mn></mfrac></mtd><mtd></mtd></mtr></mtable><mo>}</mo></mrow></math>"
    );
    
    test_parse!(
        test_text, 
        "\"b la  \"", 
        "<math display=\"block\"><mtext>b la  </mtext></math>"
    );

    test_parse!(
        test_operations, 
        "a=2", 
        "<math display=\"block\"><mi>a</mi><mo>=</mo><mn>2</mn></math>"
    );

    test_parse!(
        test_parse_group,
        "(s)",
        "<math display=\"block\"><mrow><mo>(</mo><mi>s</mi><mo>)</mo></mrow></math>"
    );

    test_parse!(
        test_function,
        "log_2 16 = 4",
        "<math display=\"block\"><msub><mi>log</mi><mn>2</mn></msub><mn>16</mn><mo>=</mo><mn>4</mn></math>"
    );

    test_parse!(
        test_fuzz_1,
        "ȳ?",
        "<math display=\"block\"><mi>ȳ</mi><mi>?</mi></math>"
    );

    test_parse!(
        test_fuzz_2,
        "{Ц\"2",
        "<math display=\"block\"><mi>{</mi><mi>Ц</mi><mi>\"</mi><mn>2</mn></math>"
    );
    
    test_parse!(
        test_fuzz_3,
        "{]!(",
        "<math display=\"block\"><mrow><mo>{</mo><mo>]</mo></mrow><mi>!</mi><mi>(</mi></math>"
    );

    test_parse!(
        test_fuzz_4,
        r#"Y{]
        ]{:}]]]"#,
        "<math display=\"block\"><mi>Y</mi><mrow><mo>{</mo><mo>]</mo></mrow><mi>\n</mi><mi>]</mi><mrow><mo>}</mo></mrow><mi>]</mi><mi>]</mi><mi>]</mi></math>"
    );

    test_parse!(
        test_ubrace_with_nested_group,
        "ubrace({(2x,+,17y,=,23),(x,-,y,=,5):})_(\"equation system\")",
        "<math display=\"block\"><munder><munder><munder><mrow><mrow><mo>{</mo><mtable columnlines=\"none none none none\"><mtr><mtd><mn>2</mn><mi>x</mi></mtd><mtd><mo>+</mo></mtd><mtd><mn>17</mn><mi>y</mi></mtd><mtd><mo>=</mo></mtd><mtd><mn>23</mn></mtd></mtr><mtr><mtd><mi>x</mi></mtd><mtd><mo>-</mo></mtd><mtd><mi>y</mi></mtd><mtd><mo>=</mo></mtd><mtd><mn>5</mn></mtd></mtr></mtable></mrow></mrow><mo>&#x23DF;</mo></munder></munder><mrow><mtext>equation system</mtext></mrow></munder></math>"
    );

    test_parse!(
        test_ubrace_with_appended_expression,
        "obrace(ubrace(t)_(a))^ba",
        "<math display=\"block\"><mover><mover><mover><mrow><munder><munder><munder><mrow><mi>t</mi></mrow><mo>&#x23DF;</mo></munder></munder><mrow><mi>a</mi></mrow></munder></mrow><mo>&#x23DE;</mo></mover></mover><mi>b</mi></mover><mi>a</mi></math>"
    );

    test_parse!(
        test_div_symbol,
        "-:",
        "<math display=\"block\"><mo>&#xF7;</mo></math>"
    );

    test_parse!(
        test_root,
        "root (abc)(d) ",
        "<math display=\"block\"><mroot><mrow><mi>d</mi></mrow><mrow><mi>a</mi><mi>b</mi><mi>c</mi></mrow></mroot></math>"
    );
}
