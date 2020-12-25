use rdcl_aoc_helpers::parse_error::ParseError;

use crate::operator::Operator;
use crate::parse_mode::ParseMode;

pub fn evaluate(mut expr: &str) -> Result<i64, ParseError> {
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
            operator = Operator::Nop;
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
