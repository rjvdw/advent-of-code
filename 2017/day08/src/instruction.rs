use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

use crate::condition::Condition;
use crate::operation::Operation;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Instruction {
    register: String,
    operation: Operation,
    condition: Condition,
}

impl Instruction {
    pub fn run(&self, register: &mut HashMap<String, i64>) {
        if self.condition.test(register) {
            let offset = self.operation.get_value();
            *register.entry(self.register.to_string()).or_insert(0) += offset;
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} if {}",
            self.register, self.operation, self.condition
        )
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut register = String::new();
        let mut operation_str = String::new();
        let mut condition_str = String::new();
        let mut parsing = Parsing::Register;

        for ch in s.chars() {
            match parsing {
                Parsing::Register => {
                    if ch == ' ' {
                        parsing = Parsing::Operation;
                    } else {
                        register.push(ch);
                    }
                }
                Parsing::Operation => {
                    if ch == ' ' && operation_str.contains(' ') {
                        parsing = Parsing::If;
                    } else {
                        operation_str.push(ch);
                    }
                }
                Parsing::If => {
                    if ch == ' ' {
                        parsing = Parsing::Condition;
                    }
                }
                Parsing::Condition => {
                    condition_str.push(ch);
                }
            }
        }

        Ok(Instruction {
            register,
            operation: operation_str.parse()?,
            condition: condition_str.parse()?,
        })
    }
}

enum Parsing {
    Register,
    Operation,
    If,
    Condition,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            "b inc 5 if a > 1".parse::<Instruction>(),
            Ok(Instruction {
                register: "b".to_string(),
                operation: Operation::Increment(5),
                condition: Condition::Gt("a".to_string(), 1),
            })
        );

        assert_eq!(
            "a inc 1 if b < 5".parse::<Instruction>(),
            Ok(Instruction {
                register: "a".to_string(),
                operation: Operation::Increment(1),
                condition: Condition::Lt("b".to_string(), 5),
            })
        );

        assert_eq!(
            "c dec -10 if a >= 1".parse::<Instruction>(),
            Ok(Instruction {
                register: "c".to_string(),
                operation: Operation::Decrement(-10),
                condition: Condition::Ge("a".to_string(), 1),
            })
        );

        assert_eq!(
            "c inc -20 if c == 10".parse::<Instruction>(),
            Ok(Instruction {
                register: "c".to_string(),
                operation: Operation::Increment(-20),
                condition: Condition::Eq("c".to_string(), 10),
            })
        );
    }
}
