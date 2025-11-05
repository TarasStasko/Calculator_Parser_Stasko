use anyhow::anyhow;
use calculator_parser::{CalculatorParser, Rule};
use pest::Parser;

#[test]
fn tests() -> anyhow::Result<()> {
    let input = "422";
    let pair = CalculatorParser::parse(Rule::expr, input)?
        .next()
        .ok_or_else(|| anyhow!("Failed"))?;
    assert_eq!(pair.as_str(), input);

    let incorr_input = "4qweqrwxa";
    assert!(CalculatorParser::parse(Rule::expr, incorr_input).is_err());
    assert!(CalculatorParser::parse(Rule::expr, "").is_err());
    Ok(())
}
