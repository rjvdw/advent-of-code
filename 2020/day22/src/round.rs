use crate::player::Player;

/// A round consists of a card, and the current state of the player.
pub type Round = (u8, Player);
