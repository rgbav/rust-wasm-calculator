use std::{iter::Peekable, slice::Iter};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvaluationError {
    #[error("syntax error - {0}")]
    SyntaxError(String),
    #[error("division by zero")]
    DivisionByZero,
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Operator(char),
    LeftParen,
    RightParen,
}

#[derive(Debug, Clone, PartialEq)]
enum Expression {
    Number(f64),
    BinaryOp {
        op: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

/// Evaluates a string expression and returns the result as f64.
/// Supports +, -, *, /, and parentheses. Returns an error for syntax errors or division by zero.
pub fn evaluate_str_expression(expression: &str) -> Result<f64, EvaluationError> {
    let tokens = tokenize(expression)?;
    let ast = parse(&tokens)?;
    let result = calculate_result(&ast)?;
    Ok(result)
}

fn tokenize(expression: &str) -> Result<Vec<Token>, EvaluationError> {
    let mut tokens = Vec::new();
    let mut chars = expression.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            // ignore whitespaces
            c if c.is_whitespace() => {
                chars.next();
            }
            '0'..='9' | '.' => {
                let mut buf = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        buf.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let n: f64 = buf
                    .parse()
                    .map_err(|_| EvaluationError::SyntaxError(format!("invalid number: {buf}")))?;
                tokens.push(Token::Number(n));
            }
            '+' | '-' | '*' | '/' => {
                tokens.push(Token::Operator(c));
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            other => {
                return Err(EvaluationError::SyntaxError(format!(
                    "invalid character: {other}"
                )));
            }
        }
    }

    Ok(tokens)
}

fn parse(tokens: &[Token]) -> Result<Expression, EvaluationError> {
    let mut iterator = tokens.iter().peekable();
    let expression = parse_expression(&mut iterator)?;
    if iterator.peek().is_some() {
        return Err(EvaluationError::SyntaxError(
            "unexpected trailing tokens".to_string(),
        ));
    }
    Ok(expression)
}

// expression -> term (('+' | '-') term)*
fn parse_expression(iter: &mut Peekable<Iter<Token>>) -> Result<Expression, EvaluationError> {
    let mut left = parse_term(iter)?;
    loop {
        let op = match iter.peek() {
            Some(Token::Operator('+')) => Operator::Add,
            Some(Token::Operator('-')) => Operator::Sub,
            _ => break,
        };
        iter.next();
        let right = parse_term(iter)?;
        left = Expression::BinaryOp {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }
    Ok(left)
}

// term -> factor (('*' | '/') factor)*
fn parse_term(iter: &mut Peekable<Iter<Token>>) -> Result<Expression, EvaluationError> {
    let mut left = parse_factor(iter)?;
    loop {
        let op = match iter.peek() {
            Some(Token::Operator('*')) => Operator::Mul,
            Some(Token::Operator('/')) => Operator::Div,
            _ => break,
        };
        iter.next();
        let right = parse_factor(iter)?;
        left = Expression::BinaryOp {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }
    Ok(left)
}

// factor -> Number | '(' expression ')'
fn parse_factor(iter: &mut Peekable<Iter<Token>>) -> Result<Expression, EvaluationError> {
    match iter.next() {
        Some(Token::Number(n)) => Ok(Expression::Number(*n)),
        Some(Token::LeftParen) => {
            let inner = parse_expression(iter)?;
            match iter.next() {
                Some(Token::RightParen) => Ok(inner),
                Some(other) => Err(EvaluationError::SyntaxError(format!(
                    "expected ')', found {other:?}"
                ))),
                None => Err(EvaluationError::SyntaxError(
                    "expected ')', found end of input".to_string(),
                )),
            }
        }
        Some(other) => Err(EvaluationError::SyntaxError(format!(
            "expected number or '(', found {other:?}"
        ))),
        None => Err(EvaluationError::SyntaxError(
            "unexpected end of input".to_string(),
        )),
    }
}

fn calculate_result(ast: &Expression) -> Result<f64, EvaluationError> {
    match ast {
        Expression::Number(n) => Ok(*n),
        Expression::BinaryOp { op, left, right } => {
            let l = calculate_result(left)?;
            let r = calculate_result(right)?;
            match op {
                Operator::Add => Ok(l + r),
                Operator::Sub => Ok(l - r),
                Operator::Mul => Ok(l * r),
                Operator::Div => {
                    if r == 0.0 {
                        Err(EvaluationError::DivisionByZero)
                    } else {
                        Ok(l / r)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_str_expression() {
        assert_eq!(evaluate_str_expression("2 + 3").unwrap(), 5.0);
        assert_eq!(evaluate_str_expression("4 * (5 - 2)").unwrap(), 12.0);
        assert_eq!(evaluate_str_expression("10 / 2").unwrap(), 5.0);
        assert!(evaluate_str_expression("1 / 0").is_err());
        assert!(evaluate_str_expression("2 +").is_err());
    }
}