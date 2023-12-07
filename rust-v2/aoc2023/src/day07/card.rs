use std::fmt;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;

const TEN: u8 = 10;
const JACK: u8 = 11;
const QUEEN: u8 = 12;
const KING: u8 = 13;
const ACE: u8 = 14;
const JOKER: u8 = 0;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Card(u8);

impl Card {
    pub fn turn_jacks_into_jokers(&mut self) {
        if self.0 == JACK {
            self.0 = JOKER;
        }
    }

    pub fn is_joker(&self) -> bool {
        self.0 == JOKER
    }

    pub fn of(value: char) -> Result<Card, ParseError> {
        match value {
            v if (0x32..=0x39).contains(&(v as u8)) => Ok(Card(v as u8 - b'0')),
            'T' => Ok(Card(TEN)),
            'J' => Ok(Card(JACK)),
            'Q' => Ok(Card(QUEEN)),
            'K' => Ok(Card(KING)),
            'A' => Ok(Card(ACE)),
            _ => err_parse_error!("invalid value while parsing card: {}", value),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            JOKER => write!(f, "J"),
            v if (2..=9).contains(&v) => write!(f, "{}", v),
            TEN => write!(f, "T"),
            JACK => write!(f, "J"),
            QUEEN => write!(f, "Q"),
            KING => write!(f, "K"),
            ACE => write!(f, "A"),
            _ => Err(fmt::Error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        assert!(Card::of('0').is_err());
        assert!(Card::of('1').is_err());
        assert_eq!(Card::of('2'), Ok(Card(2)));
        assert_eq!(Card::of('3'), Ok(Card(3)));
        assert_eq!(Card::of('4'), Ok(Card(4)));
        assert_eq!(Card::of('5'), Ok(Card(5)));
        assert_eq!(Card::of('6'), Ok(Card(6)));
        assert_eq!(Card::of('7'), Ok(Card(7)));
        assert_eq!(Card::of('8'), Ok(Card(8)));
        assert_eq!(Card::of('9'), Ok(Card(9)));
        assert_eq!(Card::of('T'), Ok(Card(TEN)));
        assert_eq!(Card::of('J'), Ok(Card(JACK)));
        assert_eq!(Card::of('Q'), Ok(Card(QUEEN)));
        assert_eq!(Card::of('K'), Ok(Card(KING)));
        assert_eq!(Card::of('A'), Ok(Card(ACE)));
        assert!(Card::of('B').is_err());
        assert!(Card::of('C').is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Card(0)), "J".to_string());
        assert_eq!(format!("{}", Card(2)), "2".to_string());
        assert_eq!(format!("{}", Card(3)), "3".to_string());
        assert_eq!(format!("{}", Card(4)), "4".to_string());
        assert_eq!(format!("{}", Card(5)), "5".to_string());
        assert_eq!(format!("{}", Card(6)), "6".to_string());
        assert_eq!(format!("{}", Card(7)), "7".to_string());
        assert_eq!(format!("{}", Card(8)), "8".to_string());
        assert_eq!(format!("{}", Card(9)), "9".to_string());
        assert_eq!(format!("{}", Card(TEN)), "T".to_string());
        assert_eq!(format!("{}", Card(JACK)), "J".to_string());
        assert_eq!(format!("{}", Card(QUEEN)), "Q".to_string());
        assert_eq!(format!("{}", Card(KING)), "K".to_string());
        assert_eq!(format!("{}", Card(ACE)), "A".to_string());
    }

    #[test]
    fn test_ord() {
        assert!(card('A') > card('K'));
        assert!(card('K') > card('Q'));
        assert!(card('Q') > card('J'));
        assert!(card('J') > card('T'));
        assert!(card('T') > card('9'));
        assert!(card('2') > joker())
    }

    fn card(value: char) -> Card {
        Card::of(value).unwrap()
    }

    fn joker() -> Card {
        Card(0)
    }
}
