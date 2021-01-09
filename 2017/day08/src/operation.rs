use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operation {
    Increment(i64),
    Decrement(i64),
}

impl Operation {
    pub fn get_value(&self) -> i64 {
        match self {
            Operation::Increment(v) => *v,
            Operation::Decrement(v) => -*v,
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Increment(v) => write!(f, "inc {}", v),
            Operation::Decrement(v) => write!(f, "dec {}", v),
        }
    }
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(r) = s.strip_prefix("inc ") {
            Ok(Operation::Increment(r.parse()?))
        } else if let Some(r) = s.strip_prefix("dec ") {
            Ok(Operation::Decrement(r.parse()?))
        } else {
            Err(ParseError(format!("Invalid operation: {}", s)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!("inc 10".parse::<Operation>(), Ok(Operation::Increment(10)));
        assert_eq!(
            "dec -10".parse::<Operation>(),
            Ok(Operation::Decrement(-10))
        );
        assert_eq!(
            "invalid".parse::<Operation>(),
            Err(ParseError::of("Invalid operation: invalid"))
        );
    }
}
