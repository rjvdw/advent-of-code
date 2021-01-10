use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

use crate::calc::helpers::partial_expression::PartialExpression;

pub struct Left<'a>(pub &'a str);

impl PartialExpression for Left<'_> {
    const OPEN_PAREN: char = ')';
    const CLOSE_PAREN: char = '(';

    fn get(&self) -> String {
        self.0.to_string()
    }

    fn has_nested_expression(&self) -> bool {
        self.0.ends_with(Left::OPEN_PAREN)
    }

    fn find(&self, ch: char) -> Option<usize> {
        self.0.rfind(ch)
    }

    fn first(a: usize, b: usize) -> usize {
        a.max(b)
    }

    fn parse_nested_expression(
        &self,
        evaluate: fn(&str) -> Result<i64, ParseError>,
    ) -> Result<(String, i64), ParseError> {
        let mut depth = 0;

        for (idx, ch) in self.0.chars().rev().enumerate() {
            if ch == Self::OPEN_PAREN {
                depth += 1;
            } else if ch == Self::CLOSE_PAREN {
                depth -= 1;
                if depth == 0 {
                    let idx = self.0.len() - 1 - idx;
                    return Ok((self.0[..idx].to_string(), evaluate(&self.0[idx..])?));
                }
            }
        }

        Err(parse_error!("Unbalanced left-hand expression: {}", self.0))
    }

    fn evaluate_number(
        &self,
        idx: usize,
        evaluate: fn(&str) -> Result<i64, ParseError>,
    ) -> Result<(String, i64), ParseError> {
        Ok((self.0[..=idx].to_string(), evaluate(&self.0[idx + 1..])?))
    }
}
