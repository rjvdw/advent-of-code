use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct InputRecord {
    pub row: u8,
    pub col: u8,
}

impl InputRecord {
    pub fn get_seat_id(&self) -> u32 {
        (self.row as u32) * 8 + (self.col as u32)
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
        let mut row = 0;
        for (i, c) in s.chars().take(7).enumerate() {
            if c == 'B' {
                row |= 2u8.pow((6 - i) as u32);
            }
        }

        let mut col = 0;
        for (i, c) in s.chars().skip(7).enumerate() {
            if c == 'R' {
                col |= 2u8.pow((2 - i) as u32);
            }
        }

        Ok(InputRecord { row, col })
    }
}