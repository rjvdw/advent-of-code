use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Player {
    position: u8,
    score: u64,
}

impl Player {
    pub fn mv(&self, roll: u64) -> Player {
        let mut player = *self;
        player.position = ((player.position as u64 + roll) % 10) as u8;
        player.score += 1 + player.position as u64;
        player
    }

    pub fn has_won(&self, target_score: u64) -> bool {
        self.score >= target_score
    }

    pub fn get_score(&self) -> u64 {
        self.score
    }
}

impl FromStr for Player {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(i) = s.rfind(' ') {
            Ok(Player {
                position: s[i + 1..].parse::<u8>()? - 1,
                score: 0,
            })
        } else {
            Err(parse_error!("Invalid input string: {}", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mv() {
        let player = Player {
            position: 0,
            score: 0,
        };

        let player = player.mv(1 + 2 + 3);
        assert_eq!(player.position, 6);
        assert_eq!(player.score, 7);

        let player = player.mv(4 + 5 + 6);
        assert_eq!(player.position, 1);
        assert_eq!(player.score, 9);
    }

    #[test]
    fn test_has_won() {
        let player = Player {
            position: 1,
            score: 750,
        };

        assert!(player.has_won(500));
        assert!(player.has_won(750));
        assert!(!player.has_won(751));
    }
}
