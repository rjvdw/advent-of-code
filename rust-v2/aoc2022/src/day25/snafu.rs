use std::collections::VecDeque;
use std::fmt;
use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;

#[derive(Debug, Eq, PartialEq)]
pub struct Snafu {
    digits: VecDeque<i8>,
}

impl Snafu {
    fn new(digits: &[i8]) -> Snafu {
        Snafu {
            digits: normalize(digits.iter().copied().collect()),
        }
    }

    pub fn as_decimal(&self) -> i64 {
        let mut decimal = 0;
        for &digit in &self.digits {
            decimal *= 5;
            decimal += digit as i64;
        }
        decimal
    }
}

impl From<i64> for Snafu {
    fn from(mut decimal: i64) -> Self {
        let mut snafu = Snafu::default();
        let mut place = 0;

        while decimal != 0 {
            place += 1;
            let digit = (decimal % 5) as i8;
            decimal /= 5;

            let mut rhs = VecDeque::with_capacity(place);
            rhs.push_back(digit);
            for _ in 1..place {
                rhs.push_back(0);
            }
            let rhs = Snafu { digits: rhs };

            snafu = &snafu + &rhs;
        }

        snafu
    }
}

impl<'a> Add<&'a Snafu> for &'a Snafu {
    type Output = Snafu;

    fn add(self, rhs: &'a Snafu) -> Self::Output {
        let mut digits = VecDeque::new();
        let mut overflow = 0;

        let mut position = self.digits.len().max(rhs.digits.len());
        let len_offset_lhs = position - self.digits.len();
        let len_offset_rhs = position - rhs.digits.len();

        while position > 0 || overflow > 0 {
            let lhs = position
                .checked_sub(1 + len_offset_lhs)
                .and_then(|p| self.digits.get(p).copied())
                .unwrap_or_default();

            let rhs = position
                .checked_sub(1 + len_offset_rhs)
                .and_then(|p| rhs.digits.get(p).copied())
                .unwrap_or_default();

            let mut sum = overflow + lhs + rhs;
            overflow = 0;

            while sum > 2 {
                // TODO: This can probably be done more efficiently
                sum -= 5;
                overflow += 1;
            }

            while sum < -2 {
                // TODO: This can probably be done more efficiently
                sum += 5;
                overflow -= 1;
            }

            digits.push_front(sum);
            position = position.saturating_sub(1);
        }

        Snafu {
            digits: normalize(digits),
        }
    }
}

fn normalize(digits: VecDeque<i8>) -> VecDeque<i8> {
    let mut digits: VecDeque<i8> = digits.into_iter().skip_while(|&d| d == 0).collect();
    if digits.is_empty() {
        digits.push_back(0);
    }

    digits
}

impl Sum for Snafu {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| &a + &b).unwrap_or_default()
    }
}

impl Default for Snafu {
    fn default() -> Self {
        Snafu::new(&[0])
    }
}

impl FromStr for Snafu {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = VecDeque::new();

        for ch in s.chars() {
            let digit = match ch {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => {
                    return err_parse_error!("Invalid digit {} in SNAFU number {}", ch, s);
                }
            };
            digits.push_back(digit);
        }

        Ok(Snafu {
            digits: normalize(digits),
        })
    }
}

impl fmt::Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &digit in &self.digits {
            write!(
                f,
                "{}",
                match digit {
                    -2 => '=',
                    -1 => '-',
                    0 => '0',
                    1 => '1',
                    2 => '2',
                    _ => panic!("Unexpected digit {digit} encountered in Snafu: {:?}", self),
                }
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(&snafu("1") + &snafu("1"), snafu("2"));
        assert_eq!(&snafu("1") + &snafu("2"), snafu("1="));
        assert_eq!(&snafu("2") + &snafu("1"), snafu("1="));
        assert_eq!(&snafu("1") + &snafu("1="), snafu("1-"));
        assert_eq!(&snafu("2") + &snafu("2"), snafu("1-"));
        assert_eq!(&snafu("1=") + &snafu("1"), snafu("1-"));
        assert_eq!(&snafu("1") + &snafu("1-"), snafu("10"));
        assert_eq!(&snafu("2") + &snafu("1-"), snafu("11"));
        assert_eq!(&snafu("1=") + &snafu("1-"), snafu("12"));
        assert_eq!(&snafu("1-") + &snafu("1-"), snafu("2="));
        assert_eq!(&snafu("20") + &snafu("1="), snafu("1=="));
    }

    #[test]
    fn test_snafu_to_decimal() {
        assert_eq!(snafu("1=-0-2").as_decimal(), 1747);
        assert_eq!(snafu("12111").as_decimal(), 906);
        assert_eq!(snafu("2=0=").as_decimal(), 198);
        assert_eq!(snafu("21").as_decimal(), 11);
        assert_eq!(snafu("2=01").as_decimal(), 201);
        assert_eq!(snafu("111").as_decimal(), 31);
        assert_eq!(snafu("20012").as_decimal(), 1257);
        assert_eq!(snafu("112").as_decimal(), 32);
        assert_eq!(snafu("1=-1=").as_decimal(), 353);
        assert_eq!(snafu("1-12").as_decimal(), 107);
        assert_eq!(snafu("12").as_decimal(), 7);
        assert_eq!(snafu("1=").as_decimal(), 3);
        assert_eq!(snafu("122").as_decimal(), 37);
    }

    #[test]
    fn test_decimal_to_snafu() {
        assert_eq!(Snafu::from(1), snafu("1"));
        assert_eq!(Snafu::from(2), snafu("2"));
        assert_eq!(Snafu::from(3), snafu("1="));
        assert_eq!(Snafu::from(4), snafu("1-"));
        assert_eq!(Snafu::from(5), snafu("10"));
        assert_eq!(Snafu::from(6), snafu("11"));
        assert_eq!(Snafu::from(7), snafu("12"));
        assert_eq!(Snafu::from(8), snafu("2="));
        assert_eq!(Snafu::from(9), snafu("2-"));
        assert_eq!(Snafu::from(10), snafu("20"));
        assert_eq!(Snafu::from(11), snafu("21"));
        assert_eq!(Snafu::from(12), snafu("22"));
        assert_eq!(Snafu::from(13), snafu("1=="));
        assert_eq!(Snafu::from(14), snafu("1=-"));
        assert_eq!(Snafu::from(15), snafu("1=0"));
        assert_eq!(Snafu::from(16), snafu("1=1"));
        assert_eq!(Snafu::from(17), snafu("1=2"));
        assert_eq!(Snafu::from(18), snafu("1-="));
        assert_eq!(Snafu::from(19), snafu("1--"));
        assert_eq!(Snafu::from(20), snafu("1-0"));
        assert_eq!(Snafu::from(2022), snafu("1=11-2"));
        assert_eq!(Snafu::from(12345), snafu("1-0---0"));
        assert_eq!(Snafu::from(314159265), snafu("1121-1110-1=0"));
    }

    fn snafu(s: &str) -> Snafu {
        s.parse().unwrap()
    }
}
