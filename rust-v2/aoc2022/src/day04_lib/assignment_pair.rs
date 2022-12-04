//! Represents a pair of assignments.

use std::str::FromStr;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;

use crate::day04_lib::assignment::Assignment;

#[derive(Debug, Eq, PartialEq)]
pub struct AssignmentPair {
    elf1: Assignment,
    elf2: Assignment,
}

impl AssignmentPair {
    pub fn contains(&self) -> bool {
        self.elf1.contains(&self.elf2) || self.elf2.contains(&self.elf1)
    }

    pub fn overlaps(&self) -> bool {
        self.elf1.overlaps(&self.elf2)
    }

    #[cfg(test)]
    fn new(elf1: (u32, u32), elf2: (u32, u32)) -> AssignmentPair {
        AssignmentPair {
            elf1: Assignment(elf1.0, elf1.1),
            elf2: Assignment(elf2.0, elf2.1),
        }
    }
}

impl FromStr for AssignmentPair {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find(',') {
            Some(pos) => {
                let elf1 = s[..pos].parse()?;
                let elf2 = s[pos + 1..].parse()?;
                Ok(AssignmentPair { elf1, elf2 })
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
        assert_eq!(
            "1-2,3-4".parse::<AssignmentPair>(),
            Ok(AssignmentPair::new((1, 2), (3, 4)))
        );
    }

    #[test]
    fn test_contains() {
        assert!(AssignmentPair::new((2, 6), (3, 4)).contains());
        assert!(AssignmentPair::new((3, 4), (2, 6)).contains());
    }

    #[test]
    fn test_not_contains() {
        assert!(!AssignmentPair::new((3, 6), (2, 4)).contains());
        assert!(!AssignmentPair::new((2, 4), (3, 6)).contains());
    }

    #[test]
    fn test_overlaps() {
        assert!(AssignmentPair::new((2, 6), (3, 4)).overlaps());
        assert!(AssignmentPair::new((3, 4), (2, 6)).overlaps());
        assert!(AssignmentPair::new((3, 6), (2, 4)).overlaps());
        assert!(AssignmentPair::new((2, 4), (3, 6)).overlaps());
    }

    #[test]
    fn test_not_overlaps() {
        assert!(!AssignmentPair::new((2, 3), (4, 5)).overlaps());
        assert!(!AssignmentPair::new((4, 5), (2, 3)).overlaps());
    }
}
