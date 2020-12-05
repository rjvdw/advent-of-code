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

impl FromStr for InputRecord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row = 0;
        let mut col = 0;
        for (i, c) in s.chars().enumerate() {
            if c == 'B' {
                row |= 2u8.pow((6 - i) as u32);
            } else if c == 'R' {
                col |= 2u8.pow((9 - i) as u32);
            }
        }

        Ok(InputRecord { row, col })
    }
}