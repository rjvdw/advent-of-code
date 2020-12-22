use helpers::from_multiline_str::FromMultilineStr;
use helpers::parse_error::ParseError;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Player {
    pub nr: usize,
    cards: Vec<u8>,
}

impl Player {
    #[cfg(test)]
    pub fn new(nr: usize, cards: &[u8]) -> Player {
        Player {
            nr,
            cards: cards.to_vec(),
        }
    }

    pub fn draw_card(&self) -> Option<(u8, Player)> {
        self.cards.split_first().map(|v| {
            (
                *v.0,
                Player {
                    nr: self.nr,
                    cards: v.1.to_vec(),
                },
            )
        })
    }

    pub fn add_cards(&self, cards: &[u8]) -> Player {
        let mut player = self.clone();
        for &card in cards {
            player.cards.push(card);
        }
        player
    }

    pub fn prepare_deck_for_sub_game(&self, nr_cards: usize) -> Player {
        Player {
            nr: self.nr,
            cards: self.cards.iter().take(nr_cards).cloned().collect(),
        }
    }

    pub fn is_eliminated(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn nr_cards(&self) -> u8 {
        self.cards.len() as u8
    }

    pub fn score(&self) -> u64 {
        let mut score = 0;
        for (idx, &card) in self.cards.iter().rev().enumerate() {
            score += (card as u64) * ((idx as u64) + 1);
        }
        score
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
