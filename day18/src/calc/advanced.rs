use helpers::ParseError;

use crate::calc::helpers::left::Left;
use crate::calc::helpers::partial_expression::PartialExpression;
use crate::calc::helpers::right::Right;
use crate::calc::simple;

pub fn evaluate(expr: &str) -> Result<i64, ParseError> {
    let mut expr = expr.to_string();
    while let Some(idx) = expr.find('+') {
        let (before, left) = evaluate_partial_expression(Left(&expr[..idx - 1]))?;
        let (after, right) = evaluate_partial_expression(Right(&expr[idx + 2..]))?;

        expr = format!("{}{}{}", before, left + right, after);
    }

    simple::evaluate(expr.as_str())
}

fn evaluate_partial_expression<T: PartialExpression>(expr: T) -> Result<(String, i64), ParseError> {
    if expr.has_nested_expression() {
        expr.parse_nested_expression(evaluate)
    } else {
        expr.find(' ')
            .map(|idx| {
                expr.find(T::CLOSE_PAREN)
                    .map(|other| T::first(idx, other))
                    .unwrap_or(idx)
            })
            .or_else(|| expr.find(T::CLOSE_PAREN))
            .map(|idx| expr.evaluate_number(idx, evaluate))
            .unwrap_or_else(|| Ok(("".to_string(), evaluate(expr.get().as_str())?)))
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
