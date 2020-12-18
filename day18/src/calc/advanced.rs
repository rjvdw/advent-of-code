use helpers::ParseError;

use crate::calc::simple;

pub fn evaluate(expr: &str) -> Result<i64, ParseError> {
    let mut expr = expr.to_string();
    while let Some(idx) = expr.find('+') {
        let (before, left) = eval_left(&expr[..idx - 1])?;
        let (after, right) = eval_right(&expr[idx + 2..])?;

        expr = format!("{}{}{}", before, left + right, after);
    }

    simple::evaluate(expr.as_str())
}

fn eval_left(expr: &str) -> Result<(&str, i64), ParseError> {
    if expr.ends_with(')') {
        let mut depth = 0;

        let ln = expr.len() - 1;
        for (idx, ch) in expr.chars().rev().enumerate() {
            let idx = ln - idx;
            match ch {
                ')' => depth += 1,
                '(' => {
                    depth -= 1;

                    if depth == 0 {
                        return Ok((&expr[..idx], evaluate(&expr[idx..])?));
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
        let idx_opt = match expr.rfind(' ') {
            Some(idx) => {
                if let Some(other) = expr.rfind('(') {
                    if idx > other {
                        Some(idx)
                    } else {
                        Some(other)
                    }
                } else {
                    Some(idx)
                }
            }
            None => expr.rfind('('),
        };

        match idx_opt {
            Some(idx) => Ok((&expr[..=idx], evaluate(&expr[idx + 1..])?)),
            None => Ok(("", evaluate(expr)?)),
        }
    }
}

fn eval_right(expr: &str) -> Result<(&str, i64), ParseError> {
    if expr.starts_with('(') {
        let mut depth = 0;

        for (idx, ch) in expr.chars().enumerate() {
            match ch {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;

                    if depth == 0 {
                        return Ok((&expr[idx + 1..], evaluate(&expr[..=idx])?));
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
        let idx_opt = match expr.find(' ') {
            Some(idx) => {
                if let Some(other) = expr.find(')') {
                    if idx < other {
                        Some(idx)
                    } else {
                        Some(other)
                    }
                } else {
                    Some(idx)
                }
            }
            None => expr.find(')'),
        };

        match idx_opt {
            Some(idx) => Ok((&expr[idx..], evaluate(&expr[..idx])?)),
            None => Ok(("", evaluate(expr)?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(evaluate("1 + 2 * 3 + 4 * 5 + 6"), Ok(231))
    }

    #[test]
    fn test_2() {
        assert_eq!(evaluate("1 + (2 * 3) + (4 * (5 + 6))"), Ok(51))
    }

    #[test]
    fn test_3() {
        assert_eq!(evaluate("2 * 3 + (4 * 5)"), Ok(46))
    }

    #[test]
    fn test_4() {
        assert_eq!(evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Ok(1445))
    }

    #[test]
    fn test_5() {
        assert_eq!(
            evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            Ok(669060)
        )
    }

    #[test]
    fn test_6() {
        assert_eq!(
            evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Ok(23340)
        )
    }
}
