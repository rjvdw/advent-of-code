use helpers::from_multiline_str::FromMultilineStr;
use helpers::parse_error::ParseError;

use crate::round::Round;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Player {
    pub nr: usize,
    cards: Vec<u8>,
}

impl Player {
    #[cfg(test)]
    pub fn new(nr: usize, cards: &[u8]) -> Player {
        let cards = cards.to_vec();
        Player { nr, cards }
    }

    pub fn draw_card(&self) -> Option<Round> {
        self.cards
            .split_first()
            .map(|(card, cards)| (*card, self.with_cards(cards)))
    }

    pub fn add_cards(&self, cards: &[u8]) -> Player {
        let mut player = self.clone();
        player.cards.extend_from_slice(cards);
        player
    }

    pub fn prepare_deck_for_sub_game(&self, nr_cards: usize) -> Player {
        let cards: Vec<u8> = self.cards.iter().take(nr_cards).cloned().collect();
        self.with_cards(&cards)
    }

    pub fn is_eliminated(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn nr_cards(&self) -> u8 {
        self.cards.len() as u8
    }

    pub fn score(&self) -> u64 {
        self.cards
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, &card)| (card as u64) * ((idx as u64) + 1))
            .sum()
    }

    fn with_cards(&self, cards: &[u8]) -> Player {
        let cards = cards.to_vec();
        Player { nr: self.nr, cards }
    }
}

impl FromMultilineStr for Player {
    const DISCARD_FIRST_RECORD: bool = true;
    type Err = ParseError;

    fn new() -> Self {
        Player {
            nr: 0,
            cards: Vec::new(),
        }
    }

    fn indicates_new_record(line: &str) -> bool {
        line.starts_with("Player")
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        if !line.is_empty() {
            if line.starts_with("Player") {
                self.nr = line[7..line.len() - 1].parse()?;
            } else {
                self.cards.push(line.parse()?);
            }
        }

        Ok(())
    }
}
