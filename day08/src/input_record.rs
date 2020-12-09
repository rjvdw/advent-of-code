use std::fmt;
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

#[derive(Debug)]
pub struct InputRecordError {
    msg: String,
}

impl fmt::Display for InputRecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl FromStr for InputRecord {
    type Err = InputRecordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = || {
            Err(InputRecordError {
                msg: format!("Invalid input line: '{}'", s),
            })
        };
        match s.find(' ') {
            Some(pos) => {
                let op: Operation = match &s[..pos] {
                    "acc" => Operation::ACC,
                    "jmp" => Operation::JMP,
                    "nop" => Operation::NOP,
                    _ => return error(),
                };
                let value: i32 = match s[pos + 1..].parse::<i32>() {
                    Ok(v) => v,
                    Err(_) => return error(),
                };

                Ok(InputRecord { op, value })
            }
            None => error(),
        }
    }
}
