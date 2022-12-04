//! Represents the assignment of a single elf.

use std::str::FromStr;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Assignment(pub(crate) u32, pub(crate) u32);

impl Assignment {
    pub fn contains(&self, other: &Assignment) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    pub fn overlaps(&self, other: &Assignment) -> bool {
        self.1 >= other.0 && other.1 >= self.0
    }
}

impl FromStr for Assignment {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find('-') {
            Some(pos) => {
                let start = s[..pos].parse()?;
                let end = s[pos + 1..].parse()?;
                Ok(Assignment(start, end))
            }
            None => err_parse_error!("Invalid input: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!("3-5".parse::<Assignment>(), Ok(Assignment(3, 5)));
    }

    #[test]
    fn test_contains() {
        assert!(Assignment(3, 5).contains(&Assignment(4, 4)));
        assert!(Assignment(3, 5).contains(&Assignment(4, 5)));
        assert!(Assignment(3, 5).contains(&Assignment(3, 4)));
        assert!(Assignment(3, 5).contains(&Assignment(3, 5)));
    }

    #[test]
    fn test_not_contains() {
        assert!(!Assignment(3, 5).contains(&Assignment(4, 6)));
        assert!(!Assignment(3, 5).contains(&Assignment(2, 4)));
        assert!(!Assignment(3, 5).contains(&Assignment(2, 3)));
        assert!(!Assignment(3, 5).contains(&Assignment(2, 4)));
        assert!(!Assignment(3, 5).contains(&Assignment(5, 8)));
        assert!(!Assignment(3, 5).contains(&Assignment(6, 8)));
    }

    #[test]
    fn test_overlaps() {
        assert!(Assignment(3, 5).overlaps(&Assignment(4, 4)));
        assert!(Assignment(3, 5).overlaps(&Assignment(4, 5)));
        assert!(Assignment(3, 5).overlaps(&Assignment(3, 4)));
        assert!(Assignment(3, 5).overlaps(&Assignment(3, 5)));
        assert!(Assignment(3, 5).overlaps(&Assignment(4, 6)));
        assert!(Assignment(3, 5).overlaps(&Assignment(5, 6)));
        assert!(Assignment(3, 5).overlaps(&Assignment(1, 3)));
        assert!(Assignment(3, 5).overlaps(&Assignment(1, 4)));
    }

    #[test]
    fn test_not_overlaps() {
        assert!(!Assignment(3, 5).overlaps(&Assignment(0, 2)));
        assert!(!Assignment(3, 5).overlaps(&Assignment(6, 8)));
    }
}
