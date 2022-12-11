//! Represents the possible operations that can be performed by the monkeys.

use std::str::FromStr;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_math::mul_mod;

/// The possible operations that can be performed by the monkeys.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operation {
    /// The monkey will add some constant to your worry level.
    Add(u64),

    /// The monkey will multiply your worry level by some constant.
    Multiply(u64),

    /// The monkey will square your worry level.
    Square,
}

impl Operation {
    /// Perform the operation on your old worry level.
    pub fn exec(&self, old: u64, modulus: u64) -> u64 {
        match self {
            Operation::Add(v) => (old + v) % modulus,
            Operation::Multiply(v) => mul_mod(old, *v, modulus),
            Operation::Square => mul_mod(old, old, modulus),
        }
    }
}

impl Default for Operation {
    fn default() -> Self {
        Operation::Add(0)
    }
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "new = old * old" {
            Ok(Operation::Square)
        } else if let Some(v) = s.strip_prefix("new = old * ") {
            Ok(Operation::Multiply(v.parse::<u64>()?))
        } else if let Some(v) = s.strip_prefix("new = old + ") {
            Ok(Operation::Add(v.parse::<u64>()?))
        } else {
            err_parse_error!("Invalid operation: {}", s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            "new = old + 1337".parse::<Operation>().unwrap(),
            Operation::Add(1337)
        );
        assert_eq!(
            "new = old * 1337".parse::<Operation>().unwrap(),
            Operation::Multiply(1337)
        );
        assert_eq!(
            "new = old * old".parse::<Operation>().unwrap(),
            Operation::Square
        );

        assert!("new = old + old".parse::<Operation>().is_err());
        assert!("new = old + new".parse::<Operation>().is_err());
        assert!("new = old * new".parse::<Operation>().is_err());

        assert!("old = old * old".parse::<Operation>().is_err());
        assert!("new=old+1".parse::<Operation>().is_err());
    }

    #[test]
    fn test_exec() {
        assert_eq!(Operation::Add(37).exec(5, 1000), 42);
        assert_eq!(Operation::Multiply(37).exec(5, 1000), 185);
        assert_eq!(Operation::Square.exec(5, 1000), 25);
    }
}
