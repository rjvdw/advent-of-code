use std::convert::TryFrom;
use std::str::FromStr;

use rdcl_aoc_helpers::err_parse_error;
use rdcl_aoc_helpers::error::ParseError;

const LOWER: u8 = b'a';
const MODULUS: u8 = 26;

#[derive(Debug)]
pub struct Room {
    name: String,
    sector: u32,
    checksum: String,
}

impl Room {
    pub fn get_sector(&self) -> u32 {
        self.sector
    }

    pub fn is_valid(&self) -> bool {
        let mut frequencies = [0u8; MODULUS as usize];

        for ch in self.name.chars() {
            if ch != '-' {
                let idx = ((ch as u8) - LOWER) as usize;
                frequencies[idx] += 1;
            }
        }

        let mut max = u8::MAX;
        let mut checksum = String::new();
        while checksum.len() < self.checksum.len() {
            let mut local_max = u8::MIN;
            for &freq in &frequencies {
                if freq < max && freq > local_max {
                    local_max = freq;
                }
            }
            for (idx, _) in frequencies
                .iter()
                .enumerate()
                .filter(|(_, &freq)| freq == local_max)
            {
                let ch = (u8::try_from(idx).unwrap() + LOWER) as char;
                checksum.push(ch);
                if checksum.len() == self.checksum.len() {
                    break;
                }
            }
            max = local_max;
        }

        checksum == self.checksum
    }

    pub fn decrypt(&self) -> String {
        let mut decrypted = String::new();
        let shift = u8::try_from(self.sector % (MODULUS as u32)).unwrap();
        for ch in self.name.chars() {
            if ch == '-' {
                decrypted.push(' ');
            } else {
                let ch = ((((ch as u8) - LOWER + shift) % MODULUS) + LOWER) as char;
                decrypted.push(ch);
            }
        }
        decrypted
    }
}

impl FromStr for Room {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let (Some(idx1), Some(idx2)) = (s.rfind('-'), s.rfind('[')) {
            let name = s[..idx1].to_string();
            let sector = s[idx1 + 1..idx2].parse()?;
            let checksum = s[idx2 + 1..s.len() - 1].to_string();

            Ok(Room {
                name,
                sector,
                checksum,
            })
        } else {
            err_parse_error!("Invalid input: {}", s)
        }
    }
}
