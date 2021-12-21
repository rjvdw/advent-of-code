extern crate rdcl_aoc_helpers;

use std::collections::HashMap;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;
use rdcl_aoc_helpers::part::Part;

use crate::deterministic_die::DeterministicDie;
use crate::game::Game;
use crate::player::Player;

mod deterministic_die;
mod game;
mod player;

/// All possible outcomes of three consecutive rolls with a Dirac die: `(outcome, frequency)`
const DIRAC_OUTCOMES: [(u64, usize); 7] = [
    (3, 1), // 111
    (4, 3), // 112 121 211
    (5, 6), // 113 131 311 122 212 221
    (6, 7), // 123 132 213 231 312 321 222
    (7, 6), // 223 232 322 133 313 331
    (8, 3), // 233 323 332
    (9, 1), // 333
];

fn main() {
    let args = get_args(&["<input file>", "<part1|part2>", "<target score>"], 1);

    let players = File::open(&args[1])
        .read_lines(1)
        .take(2) // assume only two players for simplicity
        .collect::<Vec<Player>>();
    let part = args[2].parse::<Part>().or_exit_with(1);
    let target_score = args[3].parse::<u64>().or_exit_with(1);

    match part {
        Part::One => {
            let (winning_turn, loser, rolls) = play_part_1(&players, target_score);
            println!(
                "Player #{} loses after {} rolls, making the final answer {}.",
                2 - winning_turn,
                rolls,
                loser.get_score() * (rolls as u64),
            );
        }
        Part::Two => {
            println!(
                "The winning player wins in {} universes.",
                play_part_2(&players, target_score)
            )
        }
    }
}

fn play_part_1(players: &[Player], target_score: u64) -> (usize, Player, usize) {
    let mut game = Game::new(players, target_score);
    let mut die = DeterministicDie::new(100);

    loop {
        let roll = die.next().unwrap() + die.next().unwrap() + die.next().unwrap();
        if let Some((turn, players)) = game.play(roll) {
            return (turn, players[turn ^ 1], die.get_rolls());
        }
    }
}

fn play_part_2(players: &[Player], target_score: u64) -> usize {
    let game = Game::new(players, target_score);
    let mut cache = HashMap::<Game, [usize; 2]>::new();
    let tally = play_part_2_recursive(&game, &mut cache);

    if tally[0] > tally[1] {
        tally[0]
    } else {
        tally[1]
    }
}

fn play_part_2_recursive(game: &Game, tallies: &mut HashMap<Game, [usize; 2]>) -> [usize; 2] {
    if let Some(tally) = tallies.get(game) {
        *tally
    } else {
        let mut tally = [0, 0];

        for (roll, frequency) in DIRAC_OUTCOMES {
            let mut game_copy = *game;
            if let Some((winning_turn, _)) = game_copy.play(roll) {
                tally[winning_turn] += frequency;
            } else {
                let [t1, t2] = play_part_2_recursive(&game_copy, tallies);
                tally[0] += frequency * t1;
                tally[1] += frequency * t2;
            }
        }

        tallies.insert(*game, tally);
        tally
    }
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_play_part_1() {
        let (winning_turn, loser, rolls) = play_part_1(&players(), 1000);
        assert_eq!(winning_turn, 0);
        assert_eq!(rolls, 993);
        assert_eq!(loser.get_score(), 745);
    }

    #[test]
    fn test_play_part_2() {
        assert_eq!(play_part_2(&players(), 14), 9632852745);
        assert_eq!(play_part_2(&players(), 21), 444356092776315);
    }

    fn players() -> Vec<Player> {
        vec![
            "Player 1 starting position: 4",
            "Player 2 starting position: 8",
        ]
        .as_records::<Player>()
        .unwrap()
    }
}
