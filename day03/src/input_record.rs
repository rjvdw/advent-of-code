use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct InputRecord {
    line: Vec<bool>,
}

impl InputRecord {
    pub fn test_index(&self, i: usize) -> bool {
        self.line[i % self.line.len()]
    }
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
        let line: Vec<bool> = s.chars().map(|x| x == '#').collect();
        Ok(InputRecord { line })
    }
}