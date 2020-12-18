extern crate helpers;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

use helpers::ParseError;

use crate::operator::Operator;
use crate::parse_mode::ParseMode;
use crate::Operator::Nop;

mod operator;
mod parse_mode;

/// https://adventofcode.com/2020/day/18
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", &args[0]);
        exit(1);
    }

    match read(&args[1]) {
        Ok((sum_simple, sum_advanced)) => {
            println!("Sum (simple): {}", sum_simple);
            println!("Sum (advanced): {}", sum_advanced);
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}

fn read(path: &str) -> Result<(i64, i64), ParseError> {
    let file = File::open(path)?;
    let mut sum_simple = 0;
    let mut sum_advanced = 0;

    for line in BufReader::new(file).lines() {
        let expr = line?;
        sum_simple += evaluate(expr.as_str())?;
        sum_advanced += evaluate_adv(expr.as_str())?;
    }

    Ok((sum_simple, sum_advanced))
}

fn evaluate_adv(expr: &str) -> Result<i64, ParseError> {
    let mut expr = expr.to_string();
    while let Some(pos) = expr.find('+') {
        let (before, left) = eval_left(&expr[..pos - 1])?;
        let (after, right) = eval_right(&expr[pos + 2..])?;

        let new_expr = format!("{}{}{}", before, left + right, after);
        expr = new_expr;
    }

    evaluate(expr.as_str())
}

fn eval_left(expr: &str) -> Result<(&str, i64), ParseError> {
    if expr.ends_with(')') {
        let mut depth = 0;

        let ln = expr.len() - 1;
        for (pos, ch) in expr.chars().rev().enumerate() {
            let pos = ln - pos;
            match ch {
                ')' => depth += 1,
                '(' => {
                    depth -= 1;

                    if depth == 0 {
                        return Ok((&expr[..pos], evaluate_adv(&expr[pos..])?));
                    }
                }
                _ => {}
            }
        }

        Err(ParseError(format!(
            "Unbalanced left-hand expression: {}",
            expr
        )))
    } else {
        let rpos = match expr.rfind(' ') {
            Some(pos) => {
                if let Some(o_pos) = expr.rfind('(') {
                    if pos > o_pos {
                        Some(pos)
                    } else {
                        Some(o_pos)
                    }
                } else {
                    Some(pos)
                }
            }
            None => expr.rfind('('),
        };

        match rpos {
            Some(pos) => Ok((&expr[..=pos], evaluate_adv(&expr[pos + 1..])?)),
            None => Ok(("", evaluate_adv(expr)?)),
        }
    }
}

fn eval_right(expr: &str) -> Result<(&str, i64), ParseError> {
    if expr.starts_with('(') {
        let mut depth = 0;

        for (pos, ch) in expr.chars().enumerate() {
            match ch {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;

                    if depth == 0 {
                        return Ok((&expr[pos + 1..], evaluate_adv(&expr[..=pos])?));
                    }
                }
                _ => {}
            }
        }

        Err(ParseError(format!(
            "Unbalanced right-hand expression: {}",
            expr
        )))
    } else {
        let rpos = match expr.find(' ') {
            Some(pos) => {
                if let Some(o_pos) = expr.find(')') {
                    if pos < o_pos {
                        Some(pos)
                    } else {
                        Some(o_pos)
                    }
                } else {
                    Some(pos)
                }
            }
            None => expr.find(')'),
        };

        match rpos {
            Some(pos) => Ok((&expr[pos..], evaluate_adv(&expr[..pos])?)),
            None => Ok(("", evaluate_adv(expr)?)),
        }
    }
}

fn evaluate(mut expr: &str) -> Result<i64, ParseError> {
    let mut stack: Vec<(i64, Operator)> = Vec::new();
    let mut value = 0;
    let mut operator = Operator::Nop;
    let mut mode = ParseMode::Number;

    while !expr.is_empty() {
        if expr.starts_with('(') {
            if mode == ParseMode::Operator {
                return Err(ParseError::of("Expected an operator, encountered '('."));
            }
            stack.push((value, operator));
            operator = Nop;
            expr = &expr[1..];
        } else if expr.starts_with(')') {
            if mode == ParseMode::Number {
                return Err(ParseError::of("Expected a number, encountered ')'."));
            }
            match stack.pop() {
                Some((p_value, p_operator)) => {
                    value = p_operator.evaluate(p_value, value);
                }
                None => return Err(ParseError::of("Expression is unbalanced.")),
            }
            expr = &expr[1..];
            if expr.starts_with(' ') {
                expr = &expr[1..];
            }
        } else {
            let (next, s) = find_next(expr);
            expr = next;

            mode = match mode {
                ParseMode::Number => {
                    let operand = s.parse::<i64>()?;
                    value = operator.evaluate(value, operand);
                    ParseMode::Operator
                }
                ParseMode::Operator => {
                    operator = s.parse::<Operator>()?;
                    ParseMode::Number
                }
            }
        }
    }

    if !stack.is_empty() {
        return Err(ParseError::of("Expression is unbalanced."));
    }

    Ok(value)
}

fn find_next(haystack: &str) -> (&str, &str) {
    let r1 = haystack
        .find(' ')
        .map(|idx| (idx, (&haystack[idx + 1..], &haystack[..idx])));

    let r2 = haystack
        .find(')')
        .map(|idx| (idx, (&haystack[idx..], &haystack[..idx])));

    if let Some((i1, v1)) = r1 {
        if let Some((i2, v2)) = r2 {
            if i1 < i2 {
                v1
            } else {
                v2
            }
        } else {
            v1
        }
    } else if let Some((_, v2)) = r2 {
        v2
    } else {
        ("", haystack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(evaluate("1 + 2 * 3 + 4 * 5 + 6"), Ok(71))
        }

        #[test]
        fn test_2() {
            assert_eq!(evaluate("1 + (2 * 3) + (4 * (5 + 6))"), Ok(51))
        }

        #[test]
        fn test_3() {
            assert_eq!(evaluate("2 * 3 + (4 * 5)"), Ok(26))
        }

        #[test]
        fn test_4() {
            assert_eq!(evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Ok(437))
        }

        #[test]
        fn test_5() {
            assert_eq!(
                evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
                Ok(12240)
            )
        }

        #[test]
        fn test_6() {
            assert_eq!(
                evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
                Ok(13632)
            )
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(evaluate_adv("1 + 2 * 3 + 4 * 5 + 6"), Ok(231))
        }

        #[test]
        fn test_2() {
            assert_eq!(evaluate_adv("1 + (2 * 3) + (4 * (5 + 6))"), Ok(51))
        }

        #[test]
        fn test_3() {
            assert_eq!(evaluate_adv("2 * 3 + (4 * 5)"), Ok(46))
        }

        #[test]
        fn test_4() {
            assert_eq!(evaluate_adv("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Ok(1445))
        }

        #[test]
        fn test_5() {
            assert_eq!(
                evaluate_adv("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
                Ok(669060)
            )
        }

        #[test]
        fn test_6() {
            assert_eq!(
                evaluate_adv("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
                Ok(23340)
            )
        }
    }
}
