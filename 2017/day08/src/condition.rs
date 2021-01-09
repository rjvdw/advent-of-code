use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Condition {
    Lt(String, i64),
    Gt(String, i64),
    Le(String, i64),
    Ge(String, i64),
    Eq(String, i64),
    Ne(String, i64),
}

impl Condition {
    pub fn test(&self, register: &HashMap<String, i64>) -> bool {
        match self {
            Condition::Lt(reg, right) => {
                let left = *register.get(reg).unwrap_or(&0);
                left < *right
            }
            Condition::Gt(reg, right) => {
                let left = *register.get(reg).unwrap_or(&0);
                left > *right
            }
            Condition::Le(reg, right) => {
                let left = *register.get(reg).unwrap_or(&0);
                left <= *right
            }
            Condition::Ge(reg, right) => {
                let left = *register.get(reg).unwrap_or(&0);
                left >= *right
            }
            Condition::Eq(reg, right) => {
                let left = *register.get(reg).unwrap_or(&0);
                left == *right
            }
            Condition::Ne(reg, right) => {
                let left = *register.get(reg).unwrap_or(&0);
                left != *right
            }
        }
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Condition::Lt(reg, val) => write!(f, "{} < {}", reg, val),
            Condition::Gt(reg, val) => write!(f, "{} > {}", reg, val),
            Condition::Le(reg, val) => write!(f, "{} <= {}", reg, val),
            Condition::Ge(reg, val) => write!(f, "{} >= {}", reg, val),
            Condition::Eq(reg, val) => write!(f, "{} == {}", reg, val),
            Condition::Ne(reg, val) => write!(f, "{} != {}", reg, val),
        }
    }
}

impl FromStr for Condition {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts[1] {
            "<" => Ok(Condition::Lt(parts[0].to_string(), parts[2].parse()?)),
            ">" => Ok(Condition::Gt(parts[0].to_string(), parts[2].parse()?)),
            "<=" => Ok(Condition::Le(parts[0].to_string(), parts[2].parse()?)),
            ">=" => Ok(Condition::Ge(parts[0].to_string(), parts[2].parse()?)),
            "==" => Ok(Condition::Eq(parts[0].to_string(), parts[2].parse()?)),
            "!=" => Ok(Condition::Ne(parts[0].to_string(), parts[2].parse()?)),
            _ => Err(ParseError(format!("Invalid condition: {}", s))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            "foo < 10".parse::<Condition>(),
            Ok(Condition::Lt("foo".to_string(), 10))
        );
        assert_eq!(
            "foo_bar > -10".parse::<Condition>(),
            Ok(Condition::Gt("foo_bar".to_string(), -10))
        );
        assert_eq!(
            "f != 0".parse::<Condition>(),
            Ok(Condition::Ne("f".to_string(), 0))
        );
        assert_eq!(
            "bla & 10".parse::<Condition>(),
            Err(ParseError::of("Invalid condition: bla & 10"))
        );
    }
}
