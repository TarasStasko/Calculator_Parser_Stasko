use anyhow::Result;
use stasko_calculator_parser::{CalculatorError, build_ast};

#[test]
fn test_int() -> Result<()> {
    let result = build_ast("433")?.eval()?;
    assert_eq!(result, 433);
    Ok(())
}

#[test]
fn test_ops() -> Result<()> {
    let result = build_ast("1 + 2")?.eval()?;
    assert_eq!(result, 3);
    let result = build_ast("10 - 5")?.eval()?;
    assert_eq!(result, 5);
    let result = build_ast("4 * 3")?.eval()?;
    assert_eq!(result, 12);
    let result = build_ast("10 / 2")?.eval()?;
    assert_eq!(result, 5);
    Ok(())
}

#[test]
fn test_priority() -> Result<()> {
    let result = build_ast("2 + 3 * 4")?.eval()?;
    assert_eq!(result, 14);
    let result = build_ast("2 * 3 + 4")?.eval()?;
    assert_eq!(result, 10);
    Ok(())
}

#[test]
fn test_par() -> Result<()> {
    let result = build_ast("(2 + 3) * 4")?.eval()?;
    assert_eq!(result, 20);
    let result = build_ast("2 * (3 + 4)")?.eval()?;
    assert_eq!(result, 14);
    Ok(())
}

#[test]
fn test_power() -> Result<()> {
    let result = build_ast("2 ^ 3")?.eval()?;
    assert_eq!(result, 8);
    let result = build_ast("2 ^ 3 ^ 2")?.eval()?;
    assert_eq!(result, 512);
    let result = build_ast("(2 ^ 3) ^ 2")?.eval()?;
    assert_eq!(result, 64);
    Ok(())
}

#[test]
fn test_minus() -> Result<()> {
    let result = build_ast("-5")?.eval()?;
    assert_eq!(result, -5);
    let result = build_ast("-10 + 2")?.eval()?;
    assert_eq!(result, -8);
    let result = build_ast("2 * -3")?.eval()?;
    assert_eq!(result, -6);
    let result = build_ast("-(2 + 3)")?.eval()?;
    assert_eq!(result, -5);
    let result = build_ast("-2^2")?.eval()?;
    assert_eq!(result, -4);
    let result = build_ast("(-2)^2")?.eval()?;
    assert_eq!(result, 4);
    Ok(())
}

#[test]
fn test_complex_expr() -> Result<()> {
    let result = build_ast("-5 * (1 + 2) ^ 2 - 3")?.eval()?;
    assert_eq!(result, -48);
    Ok(())
}

fn parse_and_eval(input: &str) -> Result<i64, CalculatorError> {
    build_ast(input).and_then(|ast| ast.eval())
}

#[test]
fn test_parse_err() {
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
fn test_eval_err() {
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