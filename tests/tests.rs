use stasko_calculator_parser::{CalculatorError, build_ast_from_str};

#[test]
fn test_simple_int() {
    let result = build_ast_from_str("433")
        .expect("Помилка побудови AST")
        .eval()
        .expect("Помилка обчислення");
    assert_eq!(result, 433);
}

#[test]
fn test_simple_ops() {
    let result = build_ast_from_str("1 + 2")
        .expect("Помилка побудови AST")
        .eval()
        .expect("Помилка обчислення");
    assert_eq!(result, 3);

    let result = build_ast_from_str("10 - 5")
        .expect("Помилка побудови AST")
        .eval()
        .expect("Помилка обчислення");
    assert_eq!(result, 5);

    let result = build_ast_from_str("4 * 3")
        .expect("Помилка побудови AST")
        .eval()
        .expect("Помилка обчислення");
    assert_eq!(result, 12);

    let result = build_ast_from_str("10 / 2")
        .expect("Помилка побудови AST")
        .eval()
        .expect("Помилка обчислення");
    assert_eq!(result, 5);
}

#[test]
fn test_precedence() {
    let result = build_ast_from_str("2 + 3 * 4")
        .expect("Помилка побудови AST")
        .eval()
        .expect("Помилка обчислення");
    assert_eq!(result, 14);

    let result = build_ast_from_str("2 * 3 + 4")
        .expect("Помилка побудови AST")
        .eval()
        .expect("Помилка обчислення");
    assert_eq!(result, 10);
}

#[test]
fn test_parentheses() {
    let result = build_ast_from_str("(2 + 3) * 4")
        .expect("Помилка побудови AST")
        .eval()
        .expect("Помилка обчислення");
    assert_eq!(result, 20);

    let result = build_ast_from_str("2 * (3 + 4)")
        .expect("Помилка побудови AST")
        .eval()
        .expect("Помилка обчислення");
    assert_eq!(result, 14);
}

#[test]
fn test_power() {
    let result = build_ast_from_str("2 ^ 3")
        .expect("Помилка побудови AST")
        .eval()
        .expect("Помилка обчислення");
    assert_eq!(result, 8);

    let result = build_ast_from_str("2 ^ 3 ^ 2")
        .expect("Помилка побудови AST")
        .eval()
        .expect("Помилка обчислення");
    assert_eq!(result, 512);

    let result = build_ast_from_str("(2 ^ 3) ^ 2")
        .expect("Помилка побудови AST")
        .eval()
        .expect("Помилка обчислення");
    assert_eq!(result, 64);
}

#[test]
fn test_unary_minus() {
    let result = build_ast_from_str("-5")
        .expect("Помилка побудови AST")
        .eval()
        .expect("Помилка обчислення");
    assert_eq!(result, -5);

    let result = build_ast_from_str("-10 + 2")
        .expect("Помилка побудови AST")
        .eval()
        .expect("Помилка обчислення");
    assert_eq!(result, -8);

    let result = build_ast_from_str("2 * -3")
        .expect("Помилка побудови AST")
        .eval()
        .expect("Помилка обчислення");
    assert_eq!(result, -6);

    let result = build_ast_from_str("-(2 + 3)")
        .expect("Помилка побудови AST")
        .eval()
        .expect("Помилка обчислення");
    assert_eq!(result, -5);

    let result = build_ast_from_str("-2^2")
        .expect("Помилка побудови AST")
        .eval()
        .expect("Помилка обчислення");
    assert_eq!(result, -4);

    let result = build_ast_from_str("(-2)^2")
        .expect("Помилка побудови AST")
        .eval()
        .expect("Помилка обчислення");
    assert_eq!(result, 4);
}

#[test]
fn test_complex_expression() {
    let result = build_ast_from_str("-5 * (1 + 2) ^ 2 - 3")
        .expect("Помилка побудови AST")
        .eval()
        .expect("Помилка обчислення");
    assert_eq!(result, -48);
}

fn parse_and_eval(input: &str) -> Result<i64, CalculatorError> {
    build_ast_from_str(input).and_then(|ast| ast.eval())
}

#[test]
fn test_parse_errors() {
    assert!(matches!(
        parse_and_eval("(1 + 2"),
        Err(CalculatorError::ParseError(_))
    ));
    assert!(matches!(
        parse_and_eval("1 + * 2"),
        Err(CalculatorError::ParseError(_))
    ));
    assert!(matches!(
        parse_and_eval("1 2 3"),
        Err(CalculatorError::ParseError(_))
    ));
    assert!(matches!(
        parse_and_eval(""),
        Err(CalculatorError::ParseError(_))
    ));
    assert!(matches!(
        parse_and_eval("qwe"),
        Err(CalculatorError::ParseError(_))
    ));
}

#[test]
fn test_eval_errors() {
    let err_div_zero = parse_and_eval("1 / 0");
    assert!(matches!(
        err_div_zero,
        Err(CalculatorError::EvalError(s)) if s == "Ділення на нуль"
    ));
    let err_neg_power = parse_and_eval("2 ^ -1");
    assert!(matches!(
        err_neg_power,
        Err(CalculatorError::EvalError(s)) if s.contains("Степінь не може бути від'ємним")
    ));
}
