use anyhow::{Ok, anyhow};
use stasko_calculator_parser::{CalculatorParser, Rule};
use pest::Parser;

fn main() -> anyhow::Result<()> {
    let input = "111";
    println!("Input - {input}");
    let result = CalculatorParser::parse(Rule::expr, input)?;
    let pair = result
        .clone()
        .next()
        .ok_or_else(|| anyhow!("expression not found"))?;
    println!("{:?}", pair);

    Ok(())
}
