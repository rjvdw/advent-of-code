use rdcl_aoc_helpers::parse_error::ParseError;

use crate::calc::helpers::partial_expression::PartialExpression;

pub struct Right<'a>(pub &'a str);

impl PartialExpression for Right<'_> {
    const OPEN_PAREN: char = '(';
    const CLOSE_PAREN: char = ')';

    fn get(&self) -> String {
        self.0.to_string()
    }

    fn has_nested_expression(&self) -> bool {
        self.0.starts_with(Right::OPEN_PAREN)
    }

    fn find(&self, ch: char) -> Option<usize> {
        self.0.find(ch)
    }

    fn first(a: usize, b: usize) -> usize {
        a.min(b)
    }

    fn parse_nested_expression(
        &self,
        evaluate: fn(&str) -> Result<i64, ParseError>,
    ) -> Result<(String, i64), ParseError> {
        let mut depth = 0;

        for (idx, ch) in self.0.chars().enumerate() {
            if ch == Self::OPEN_PAREN {
                depth += 1;
            } else if ch == Self::CLOSE_PAREN {
                depth -= 1;
                if depth == 0 {
                    return Ok((self.0[idx + 1..].to_string(), evaluate(&self.0[..=idx])?));
                }
            }
        }

        Err(ParseError(format!(
            "Unbalanced right-hand expression: {}",
            self.0
        )))
    }

    fn evaluate_number(
        &self,
        idx: usize,
        evaluate: fn(&str) -> Result<i64, ParseError>,
    ) -> Result<(String, i64), ParseError> {
        Ok((self.0[idx..].to_string(), evaluate(&self.0[..idx])?))
    }
}
