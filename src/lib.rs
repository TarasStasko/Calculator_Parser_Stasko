//! # Calculator Parser
//!
//! ## Граматика Парсера
//!
//! ```pest
//! Ігноруємо пробіли, табуляцію та нові рядки.
//! WHITESPACE = _{ " " | "\t" | "\n" }
//!
//! Коректне число — це послідовність ASCII цифр
//! int = @{ ASCII_DIGIT+ }
//!
//! Оператор додавання
//! add = { "+" }
//!
//! Оператор віднімання
//! subtract = { "-" }
//!
//! Оператор множення
//! multiply = { "*" }
//!
//! Оператор ділення
//! divide = { "/" }
//!
//! Оператор піднесення до степеня
//! power = { "^" }
//!
//! primary - це числа або вирази в дужках.
//! primary = { int | "(" ~ expr ~ ")" }
//!
//! factor - обробляє унарні оператори(унарний мінус)
//! factor = { (subtract)? ~ power_term }
//!
//! power_term - обробляє піднесення до степеня (право-асоціативне)
//! power_term = { primary ~ (power ~ factor)* }
//!
//! term - обробляє множення та ділення (ліво-асоціативне)
//! term = { factor ~ ((multiply | divide) ~ factor)* }
//!
//! expr - обробляє додавання та віднімання (ліво-асоціативне)
//! expr = { term ~ ((add | subtract) ~ term)* }
//!
//! expression - кореневе правило, expr має покрити весь вхід
//! expression = { SOI ~ expr ~ EOI }
//! ```

use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct CalculatorParser;

/// Кастомні помилки
#[derive(Error, Debug, PartialEq)]
pub enum CalculatorError {
    #[error("Помилка парсингу: {0}")]
    ParseError(#[from] Box<pest::error::Error<Rule>>),
    #[error("Помилка обчислення: {0}")]
    EvalError(String),
}

/// Оператори, які підтримує калькулятор
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

/// Структура абстрактного дерева
#[derive(Debug, Clone, PartialEq)]
pub enum ExprAst {
    Int(i64),
    UnaryOp {
        op: Op,
        child: Box<ExprAst>,
    },
    BinaryOp {
        op: Op,
        lhs: Box<ExprAst>,
        rhs: Box<ExprAst>,
    },
}

/// Головна функція, що перетворює вхідний рядок на AST
pub fn build_ast(input: &str) -> Result<ExprAst, CalculatorError> {
    let pair = CalculatorParser::parse(Rule::expression, input)
        .map_err(|e| CalculatorError::ParseError(Box::new(e)))?
        .next()
        .ok_or_else(|| CalculatorError::EvalError("Не знайдено виразу".to_string()))?;
    let inner_expr = pair.into_inner().next().unwrap();
    build_ast_from_pair(inner_expr)
}

/// Допоміжна функція для згортання ліво-асоціативних правил
pub fn build_left_assoc(
    mut pairs: pest::iterators::Pairs<Rule>,
) -> Result<ExprAst, CalculatorError> {
    let mut ast = build_ast_from_pair(pairs.next().unwrap())?;
    while let (Some(op_pair), Some(rhs_pair)) = (pairs.next(), pairs.next()) {
        let op = match op_pair.as_rule() {
            Rule::add => Op::Add,
            Rule::subtract => Op::Subtract,
            Rule::multiply => Op::Multiply,
            Rule::divide => Op::Divide,
            _ => unreachable!(),
        };
        let rhs = build_ast_from_pair(rhs_pair)?;
        ast = ExprAst::BinaryOp {
            op,
            lhs: Box::new(ast),
            rhs: Box::new(rhs),
        };
    }
    Ok(ast)
}

/// Рекурсивна функція для побудови AST з pest::Pair
pub fn build_ast_from_pair(pair: pest::iterators::Pair<Rule>) -> Result<ExprAst, CalculatorError> {
    match pair.as_rule() {
        Rule::expr | Rule::term => build_left_assoc(pair.into_inner()),
        Rule::power_term => {
            let mut pairs = pair.into_inner().collect::<Vec<_>>();
            let mut ast = build_ast_from_pair(pairs.pop().unwrap())?;
            while let (Some(op_pair), Some(lhs_pair)) = (pairs.pop(), pairs.pop()) {
                assert_eq!(op_pair.as_rule(), Rule::power);
                let op = Op::Power;
                let lhs = build_ast_from_pair(lhs_pair)?;
                ast = ExprAst::BinaryOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(ast),
                };
            }
            Ok(ast)
        }
        Rule::factor => {
            let mut inner = pair.into_inner();
            let first = inner.next().unwrap();
            if first.as_rule() == Rule::subtract {
                let child = build_ast_from_pair(inner.next().unwrap())?;
                Ok(ExprAst::UnaryOp {
                    op: Op::Subtract,
                    child: Box::new(child),
                })
            } else {
                build_ast_from_pair(first)
            }
        }
        Rule::primary => {
            let inner = pair.into_inner().next().unwrap();
            match inner.as_rule() {
                Rule::int => {
                    let i: i64 = inner.as_str().parse().unwrap();
                    Ok(ExprAst::Int(i))
                }
                Rule::expr => build_ast_from_pair(inner),
                _ => unreachable!(),
            }
        }
        _ => unreachable!("Неочікуване правило: {:?}", pair.as_rule()),
    }
}

impl ExprAst {
    /// Рекурсивна функція для обчислення значення виразу
    pub fn eval(self) -> Result<i64, CalculatorError> {
        match self {
            ExprAst::Int(i) => Ok(i),
            ExprAst::UnaryOp { op, child } => {
                let child_val = child.eval()?;
                match op {
                    Op::Subtract => Ok(-child_val),
                    _ => Err(CalculatorError::EvalError(format!(
                        "Непідтримувана унарна операція: {:?}",
                        op
                    ))),
                }
            }
            ExprAst::BinaryOp { op, lhs, rhs } => {
                let lhs_val = lhs.eval()?;
                let rhs_val = rhs.eval()?;
                match op {
                    Op::Add => Ok(lhs_val + rhs_val),
                    Op::Subtract => Ok(lhs_val - rhs_val),
                    Op::Multiply => Ok(lhs_val * rhs_val),
                    Op::Divide => {
                        if rhs_val == 0 {
                            Err(CalculatorError::EvalError("Ділення на нуль".to_string()))
                        } else {
                            Ok(lhs_val / rhs_val)
                        }
                    }
                    Op::Power => {
                        if rhs_val < 0 {
                            Err(CalculatorError::EvalError(
                                "Степінь не може бути від'ємним".to_string(),
                            ))
                        } else {
                            Ok(lhs_val.pow(rhs_val as u32))
                        }
                    }
                }
            }
        }
    }
}
