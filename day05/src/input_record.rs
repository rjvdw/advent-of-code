use std::str::FromStr;

const ROW_BIT_FLAGS: [u8; 7] = [
    0b01000000,
    0b00100000,
    0b00010000,
    0b00001000,
    0b00000100,
    0b00000010,
    0b00000001,
];

const COL_BIT_FLAGS: [u8; 3] = [
    0b00000100,
    0b00000010,
    0b00000001,
];

#[derive(Debug)]
pub struct InputRecord {
    pub row: u8,
    pub col: u8,
}

impl InputRecord {
    pub fn get_seat_id(&self) -> u16 {
        (self.row as u16) * 8 + (self.col as u16)
    }
}

impl FromStr for InputRecord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row = 0;
        let mut col = 0;
        for (i, c) in s.chars().enumerate() {
            if c == 'B' {
                row |= ROW_BIT_FLAGS[i];
            } else if c == 'R' {
                col |= COL_BIT_FLAGS[i - 7];
            }
        }

        Ok(InputRecord { row, col })
    }
}
