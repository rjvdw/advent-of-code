use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::{assert_or_parse_error, ParseResult};

use crate::card::Card;

pub const HAND_SIZE: usize = 5;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Hand([Card; HAND_SIZE]);

impl Hand {
    pub fn turn_jacks_into_jokers(&mut self) {
        for card in self.0.iter_mut() {
            card.turn_jacks_into_jokers()
        }
    }

    pub fn get_type(&self) -> HandType {
        let mut counts_per_card: HashMap<Card, usize> = HashMap::new();
        let mut joker_count = 0;
        for card in self.0 {
            *counts_per_card.entry(card).or_insert(0) += 1;
            if card.is_joker() {
                joker_count += 1;
            }
        }
        let mut values = counts_per_card.values().copied().collect::<Vec<_>>();
        values.sort();

        match values.iter().max().unwrap() {
            // AAAAA (or JJJJJ)
            5 => HandType::FiveOfAKind,
            // JJJJA
            _ if joker_count == 4 => HandType::FiveOfAKind,
            // AAAAJ
            4 if joker_count == 1 => HandType::FiveOfAKind,
            // AAAJJ
            3 if joker_count == 2 => HandType::FiveOfAKind,
            // JJAAA
            3 if joker_count == 3 && values[0] == 2 => HandType::FiveOfAKind,

            // AAAAx
            4 => HandType::FourOfAKind,
            // AAAJx
            3 if joker_count == 1 => HandType::FourOfAKind,
            // JJJAx
            _ if joker_count == 3 => HandType::FourOfAKind,
            // AAJJx
            2 if values[1] == 2 && joker_count == 2 => HandType::FourOfAKind,

            // AAAKK
            3 if values[0] == 2 => HandType::FullHouse,
            // AAKKJ
            2 if values[1] == 2 && joker_count == 1 => HandType::FullHouse,

            // AAAxx
            3 => HandType::ThreeOfAKind,
            // AAJxx
            2 if joker_count == 1 => HandType::ThreeOfAKind,
            _ if joker_count == 2 => HandType::ThreeOfAKind,

            // AAKKx
            2 if values[1] == 2 => HandType::TwoPair,

            // AAxxx
            2 => HandType::OnePair,
            // AJxxx
            _ if joker_count == 1 => HandType::OnePair,

            // xxxxx
            _ => HandType::HighCard,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_type().cmp(&other.get_type()).then_with(|| {
            for i in 0..HAND_SIZE {
                match self.0[i].cmp(&other.0[i]) {
                    Ordering::Less => {
                        return Ordering::Less;
                    }
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        return Ordering::Greater;
                    }
                }
            }
            Ordering::Equal
        })
    }
}

impl FromStr for Hand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_or_parse_error!(
            s.len() == HAND_SIZE,
            "Received hand with invalid size (must be {}): {}",
            HAND_SIZE,
            s
        );

        let cards = s.chars().map(Card::of).collect::<ParseResult<Vec<_>>>()?;

        Ok(Hand(cards.try_into().unwrap()))
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in self.0 {
            write!(f, "{card}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum HandType {
    /// High card, where all cards' labels are distinct: 23456
    HighCard,

    /// One pair, where two cards share one label, and the other three
    /// cards have a different label from the pair and each other:
    /// A23A4
    OnePair,

    /// Two pair, where two cards share one label, two other cards
    /// share a second label, and the remaining card has a third label:
    /// 23432
    TwoPair,

    /// Three of a kind, where three cards have the same label, and the
    /// remaining two cards are each different from any other card in
    /// the hand: TTT98
    ThreeOfAKind,

    /// Full house, where three cards have the same label, and the
    /// remaining two cards share a different label: 23332
    FullHouse,

    /// Four of a kind, where four cards have the same label and one
    /// card has a different label: AA8AA
    FourOfAKind,

    /// Five of a kind, where all five cards have the same label: AAAAA
    FiveOfAKind,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_without_jokers() {
        assert_eq!(hand("AAAAA").get_type(), HandType::FiveOfAKind);
        assert_eq!(hand("AAAA2").get_type(), HandType::FourOfAKind);
        assert_eq!(hand("AAA2A").get_type(), HandType::FourOfAKind);
        assert_eq!(hand("AA2AA").get_type(), HandType::FourOfAKind);
        assert_eq!(hand("A2AAA").get_type(), HandType::FourOfAKind);
        assert_eq!(hand("2AAAA").get_type(), HandType::FourOfAKind);
        assert_eq!(hand("2A2A2").get_type(), HandType::FullHouse);
        assert_eq!(hand("22AA2").get_type(), HandType::FullHouse);
        assert_eq!(hand("222AA").get_type(), HandType::FullHouse);
        assert_eq!(hand("222AK").get_type(), HandType::ThreeOfAKind);
        assert_eq!(hand("K222A").get_type(), HandType::ThreeOfAKind);
        assert_eq!(hand("22334").get_type(), HandType::TwoPair);
        assert_eq!(hand("22343").get_type(), HandType::TwoPair);
        assert_eq!(hand("22433").get_type(), HandType::TwoPair);
        assert_eq!(hand("24233").get_type(), HandType::TwoPair);
        assert_eq!(hand("42233").get_type(), HandType::TwoPair);
        assert_eq!(hand("23432").get_type(), HandType::TwoPair);
        assert_eq!(hand("AA234").get_type(), HandType::OnePair);
        assert_eq!(hand("23456").get_type(), HandType::HighCard);
    }

    #[test]
    fn test_type_with_jokers() {
        assert_eq!(handj("AAAAJ").get_type(), HandType::FiveOfAKind);
        assert_eq!(handj("AAAJJ").get_type(), HandType::FiveOfAKind);
        assert_eq!(handj("AAJJJ").get_type(), HandType::FiveOfAKind);
        assert_eq!(handj("AJJJJ").get_type(), HandType::FiveOfAKind);
        assert_eq!(handj("JJJJJ").get_type(), HandType::FiveOfAKind);

        assert_eq!(handj("AAAJK").get_type(), HandType::FourOfAKind);
        assert_eq!(handj("AAJJK").get_type(), HandType::FourOfAKind);
        assert_eq!(handj("AJJJK").get_type(), HandType::FourOfAKind);

        assert_eq!(handj("AAKKJ").get_type(), HandType::FullHouse);

        assert_eq!(handj("AAJKQ").get_type(), HandType::ThreeOfAKind);
        assert_eq!(handj("AJJKQ").get_type(), HandType::ThreeOfAKind);

        assert_eq!(handj("AKQJT").get_type(), HandType::OnePair);
    }

    #[test]
    fn test_ord_hand() {
        assert!(hand("KKKKK") > hand("AAAAK"));
        assert!(hand("AAAAA") > hand("KKKKK"));
        assert!(hand("AKKKK") > hand("KAAAA"));
        assert!(hand("33332") > hand("2AAAA"));
        assert!(hand("77888") > hand("77788"));
    }

    #[test]
    fn test_ord_hand_type() {
        assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
        assert!(HandType::FourOfAKind > HandType::FullHouse);
        assert!(HandType::FullHouse > HandType::ThreeOfAKind);
        assert!(HandType::ThreeOfAKind > HandType::TwoPair);
        assert!(HandType::TwoPair > HandType::OnePair);
        assert!(HandType::OnePair > HandType::HighCard);
    }

    fn hand(s: &str) -> Hand {
        s.parse().unwrap()
    }

    fn handj(s: &str) -> Hand {
        let mut hand: Hand = s.parse().unwrap();
        hand.turn_jacks_into_jokers();
        hand
    }
}
