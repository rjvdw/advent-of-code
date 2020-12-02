use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct InputRecord {
    idx1: usize,
    idx2: usize,
    character: char,
    password: String,
}

impl InputRecord {
    pub fn valid_according_to_old_job(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.character).count();

        count >= self.idx1 && count <= self.idx2
    }

    pub fn valid_according_to_corporate_policy(&self) -> bool {
        let c1 = self.password.chars().nth(self.idx1 - 1).filter(|&c| c == self.character);
        let c2 = self.password.chars().nth(self.idx2 - 1).filter(|&c| c == self.character);

        (c1.is_some() && c2.is_none()) || (c1.is_none() && c2.is_some())
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
        let p1 = match s.chars().position(|c| c == '-') {
            Some(pos) => pos,
            None => return Err(InputRecordError { msg: format!("Input file has invalid format in line {}", s) })
        };
        let p2 = p1 + match s.chars().skip(p1).position(|c| c == ' ') {
            Some(pos) => pos,
            None => return Err(InputRecordError { msg: format!("Input file has invalid format in line {}", s) })
        };
        let p3 = p2 + match s.chars().skip(p2).position(|c| c == ':') {
            Some(pos) => pos,
            None => return Err(InputRecordError { msg: format!("Input file has invalid format in line {}", s) })
        };

        let idx1 = match s[..p1].parse::<usize>() {
            Ok(val) => val,
            Err(_) => return Err(InputRecordError { msg: format!("Invalid lower bound in line {}", s) })
        };
        let idx2 = match s[p1 + 1..p2].parse::<usize>() {
            Ok(val) => val,
            Err(_) => return Err(InputRecordError { msg: format!("Invalid upper bound in line {}", s) })
        };
        let character = match s.chars().nth(p2 + 1) {
            Some(val) => val,
            None => return Err(InputRecordError { msg: format!("Invalid character in line {}", s) })
        };
        let password = s.chars().skip(p3 + 2).collect();

        Ok(InputRecord { idx1, idx2, character, password })
    }
}