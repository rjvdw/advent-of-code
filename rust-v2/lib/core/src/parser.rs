//! Input parser.

use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

use crate::{err_parse_error, ParseResult};

pub struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser { input, pos: 0 }
    }

    pub fn exhausted(&self) -> bool {
        self.pos >= self.input.len()
    }

    pub fn rest(&self) -> &str {
        &self.input[self.pos..]
    }

    pub fn skip_text(&mut self, text: &str) -> ParseResult<()> {
        if self.exhausted() {
            err_parse_error!("Input string is exhausted.")
        } else if self.rest().starts_with(text) {
            self.pos += text.len();
            Ok(())
        } else {
            err_parse_error!(
                "Input '{}' does not contain text '{}' at position {}",
                self.input,
                text,
                self.pos
            )
        }
    }

    pub fn take_value<T>(&mut self) -> ParseResult<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        if self.exhausted() {
            err_parse_error!("Input string is exhausted.")
        } else {
            match self.rest().parse::<T>() {
                Ok(v) => {
                    self.pos = self.input.len();
                    Ok(v)
                }
                Err(e) => err_parse_error!(
                    "Could not parse value '{}' with error '{:?}'",
                    self.rest(),
                    e
                ),
            }
        }
    }

    pub fn take_value_upto<T>(&mut self, suffix: &str) -> ParseResult<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        if self.exhausted() {
            err_parse_error!("Input string is exhausted.")
        } else {
            match self.rest().find(suffix) {
                Some(idx) => {
                    let value = &self.input[self.pos..self.pos + idx];
                    match value.parse::<T>() {
                        Ok(v) => {
                            self.pos += idx + suffix.len();
                            Ok(v)
                        }
                        Err(e) => err_parse_error!(
                            "Could not parse value '{}' with error '{:?}'",
                            self.rest(),
                            e
                        ),
                    }
                }
                None => err_parse_error!(
                    "Input '{}' does not contain suffix '{}' at position {}",
                    self.input,
                    suffix,
                    self.pos
                ),
            }
        }
    }
}

/// Parse a string as a whitespace separated list of values into a Vec.
pub fn parse_whitespace_separated_to_vec<T>(line: &str) -> Result<Vec<T>, T::Err>
where
    T: FromStr,
{
    line.trim()
        .split_ascii_whitespace()
        .map(|v| v.parse::<T>())
        .collect::<Result<Vec<T>, T::Err>>()
}

/// Parse a string as a whitespace separated list of values into a HashSet.
pub fn parse_whitespace_separated_to_hashset<T>(line: &str) -> Result<HashSet<T>, T::Err>
where
    T: FromStr + Hash + Eq,
{
    line.trim()
        .split_ascii_whitespace()
        .map(|v| v.parse::<T>())
        .collect::<Result<HashSet<T>, T::Err>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_success() {
        let input = "This is some text with value 15 embedded in it. It also contains value 7. It ends with value 10";
        let mut parser = Parser::new(input);

        parser.skip_text("This is some text with value ").unwrap();
        let v1 = parser.take_value_upto::<u8>(" embedded in it. ").unwrap();
        parser.skip_text("It also contains value ").unwrap();
        let v2 = parser.take_value_upto::<u8>(". ").unwrap();
        parser.skip_text("It ends with value ").unwrap();
        let v3 = parser.take_value::<u8>().unwrap();

        assert_eq!(v1, 15);
        assert_eq!(v2, 7);
        assert_eq!(v3, 10);
    }

    #[test]
    fn test_parser_cannot_find_text() {
        let input = "Some input";
        let mut parser = Parser::new(input);

        let err = parser.skip_text("invalid text").unwrap_err();

        assert_eq!(
            &err.0,
            "Input 'Some input' does not contain text 'invalid text' at position 0"
        );
    }

    #[test]
    fn test_parser_cannot_find_suffix() {
        let input = "Some input";
        let mut parser = Parser::new(input);

        let err = parser.take_value_upto::<u8>("invalid text").unwrap_err();

        assert_eq!(
            &err.0,
            "Input 'Some input' does not contain suffix 'invalid text' at position 0"
        );
    }

    #[test]
    fn test_parser_cannot_parse_value() {
        let input = "Some input";
        let mut parser = Parser::new(input);

        let err = parser.take_value_upto::<u8>(" input").unwrap_err();

        assert_eq!(
            &err.0,
            "Could not parse value 'Some input' with error 'ParseIntError { kind: InvalidDigit }'"
        );
    }

    #[test]
    fn test_parse_whitespace_separated_to_vec_1() {
        let line = "10  20  30";
        let parsed = parse_whitespace_separated_to_vec::<u32>(line);
        assert_eq!(parsed, Ok(vec![10, 20, 30]));
    }

    #[test]
    fn test_parse_whitespace_separated_to_vec_2() {
        let line = "  10  20  30  ";
        let parsed = parse_whitespace_separated_to_vec::<u32>(line);
        assert_eq!(parsed, Ok(vec![10, 20, 30]));
    }

    #[test]
    fn test_parse_whitespace_separated_to_vec_3() {
        let line = "10  20  asd";
        let parsed = parse_whitespace_separated_to_vec::<u32>(line);
        assert!(parsed.is_err());
    }

    #[test]
    fn test_parse_whitespace_separated_to_hashset_1() {
        let line = "10  20  30";
        let parsed = parse_whitespace_separated_to_hashset::<u32>(line);
        assert_eq!(parsed, Ok(HashSet::from([10, 20, 30])));
    }

    #[test]
    fn test_parse_whitespace_separated_to_hashset_2() {
        let line = "  10  20  30  ";
        let parsed = parse_whitespace_separated_to_hashset::<u32>(line);
        assert_eq!(parsed, Ok(HashSet::from([10, 20, 30])));
    }

    #[test]
    fn test_parse_whitespace_separated_to_hashset_3() {
        let line = "10  20  asd";
        let parsed = parse_whitespace_separated_to_hashset::<u32>(line);
        assert!(parsed.is_err());
    }
}
