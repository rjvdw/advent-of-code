extern crate helpers;

use std::collections::HashSet;
use std::env;
use std::process::exit;

use helpers::handle_result;
use helpers::read::read_multiline_input;

use crate::player::Player;

mod player;

/// A round consists of a card, and the current state of the player.
type Round = (u8, Player);

/// https://adventofcode.com/2020/day/22
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", &args[0]);
        exit(1);
    }

    let game_state = handle_result(read_multiline_input::<Player>(&args[1]));

    match play_combat(&game_state) {
        Some(winner) => println!(
            "The winner of Combat is {}, with a score of: {}",
            winner.nr,
            winner.score()
        ),
        None => println!("Nobody won Combat"),
    }

    match play_recursive_combat(&game_state) {
        Some(winner) => println!(
            "The winner of Recursive Combat is {}, with a score of: {}",
            winner.nr,
            winner.score()
        ),
        None => println!("Nobody won Recursive Combat"),
    }
}

fn play_combat(initial_game_state: &[Player]) -> Option<Player> {
    let mut game_state = initial_game_state.to_vec();
    loop {
        let mut next_game_state: Vec<Player> = Vec::new();
        let mut cards: Vec<u8> = Vec::new();
        let mut winner: Option<(Round, usize)> = None;
        let mut idx = 0;

        for player in game_state {
            if !player.is_eliminated() {
                let current_round = player.draw_card().unwrap();
                next_game_state.push(current_round.1.clone());
                cards.push(current_round.0);
                winner = check_highest_card(&winner, &current_round, idx);
                idx += 1;
            }
        }

        match winner {
            Some(((card, player), idx)) => {
                for i in 1..=idx {
                    cards[i] = cards[i - 1];
                }
                cards[0] = card;
                next_game_state[idx] = player.add_cards(&cards);
            }
            None => return None,
        }

        game_state = next_game_state;
        let remaining_players = get_remaining_players(&game_state);
        if remaining_players.len() == 1 {
            return Some(remaining_players[0].clone());
        }
    }
}

fn play_recursive_combat(initial_game_state: &[Player]) -> Option<Player> {
    let mut seen: HashSet<Player> = HashSet::new();
    let mut game_state = initial_game_state.to_vec();
    loop {
        let mut next_game_state: Vec<Player> = Vec::new();
        let mut rounds: Vec<Round> = Vec::new();
        let mut winner: Option<(Round, usize)> = None;
        let mut enough_cards_remaining = true;

        for player in &game_state {
            if !player.is_eliminated() {
                // if there was a previous round in this game that had exactly the same cards in the
                // same order in the same players' decks, the game instantly ends in a win for
                // player 1
                if seen.contains(&player) {
                    return game_state.iter().find(|player| player.nr == 1).cloned();
                }
                seen.insert(player.clone());

                // the players begin the round by each drawing the top card of their deck as normal
                let (card, player) = player.draw_card().unwrap();
                next_game_state.push(player.clone());
                rounds.push((card, player.clone()));

                if player.nr_cards() < card {
                    enough_cards_remaining = false;
                }
            }
        }

        if enough_cards_remaining {
            // If both players have at least as many cards remaining in their deck as the value of
            // the card they just drew, the winner of the round is determined by playing a new game
            // of Recursive Combat.

            let mut sub_game_state: Vec<Player> = Vec::new();
            for (card, player) in &rounds {
                sub_game_state.push(player.prepare_deck_for_sub_game(*card as usize));
            }

            match play_recursive_combat(&sub_game_state) {
                Some(player) => {
                    for (idx, current_round) in rounds.iter().enumerate() {
                        if current_round.1.nr == player.nr {
                            winner = Some((current_round.clone(), idx));
                        }
                    }
                }
                None => return None,
            }
        } else {
            // Otherwise, at least one player must not have enough cards left in their deck to
            // recurse; the winner of the round is the player with the higher-value card.
            for (idx, current_round) in rounds.iter().enumerate() {
                winner = check_highest_card(&winner, current_round, idx);
            }
        }

        match winner {
            Some((round, idx)) => {
                let mut cards: Vec<u8> = Vec::new();
                cards.push(round.0);

                for (round_idx, round) in rounds.iter().enumerate() {
                    if round_idx != idx {
                        cards.push(round.0);
                    }
                }

                next_game_state[idx] = round.1.add_cards(&cards);
            }
            None => return None,
        }

        game_state = next_game_state;
        let remaining_players = get_remaining_players(&game_state);
        if remaining_players.len() == 1 {
            return Some(remaining_players[0].clone());
        }
    }
}

fn get_remaining_players(game_state: &[Player]) -> Vec<Player> {
    game_state
        .iter()
        .filter(|player| !player.is_eliminated())
        .cloned()
        .collect()
}

fn check_highest_card(
    winner_so_far: &Option<(Round, usize)>,
    current_round: &Round,
    idx: usize,
) -> Option<(Round, usize)> {
    match winner_so_far {
        Some((winning_round, _)) => {
            if current_round.0 > winning_round.0 {
                Some((current_round.clone(), idx))
            } else {
                winner_so_far.clone()
            }
        }
        None => Some((current_round.clone(), idx)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combat() {
        let game_state = vec![
            Player::new(1, &[9, 2, 6, 3, 1]),
            Player::new(2, &[5, 8, 4, 7, 10]),
        ];

        let winner = play_combat(&game_state);

        assert!(winner.is_some());

        let winner = winner.unwrap();
        assert_eq!((winner.nr, winner.score()), (2, 306));
    }

    #[test]
    fn test_recursive_combat_infinite() {
        let game_state = vec![Player::new(1, &[43, 19]), Player::new(2, &[2, 29, 14])];

        let winner = play_recursive_combat(&game_state);

        assert!(winner.is_some());

        let winner = winner.unwrap();
        assert_eq!((winner.nr, winner.score()), (1, 105));
    }

    #[test]
    fn test_recursive_combat() {
        let game_state = vec![
            Player::new(1, &[9, 2, 6, 3, 1]),
            Player::new(2, &[5, 8, 4, 7, 10]),
        ];

        let winner = play_recursive_combat(&game_state);

        assert!(winner.is_some());

        let winner = winner.unwrap();
        assert_eq!((winner.nr, winner.score()), (2, 291));
    }
}
