use helpers::ParseError;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    ACC,
    JMP,
    NOP,
}

#[derive(Debug, Copy, Clone)]
pub struct InputRecord {
    pub op: Operation,
    pub value: i32,
}

impl FromStr for InputRecord {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find(' ') {
            Some(pos) => {
                let op: Operation = match &s[..pos] {
                    "acc" => Operation::ACC,
                    "jmp" => Operation::JMP,
                    "nop" => Operation::NOP,
                    _ => return Err(ParseError(format!("Invalid input line: '{}'", s))),
                };
                let value: i32 = s[pos + 1..].parse::<i32>()?;

                Ok(InputRecord { op, value })
            }
            None => Err(ParseError(format!("Invalid input line: '{}'", s))),
        }
    }
}
