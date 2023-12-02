//! The solution for [advent of code 2023, day 2](https://adventofcode.com/2023/day/2)

use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::MainResult;

use crate::game::{Game, Grab};

mod game;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 2")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The set of cubes to test against.
    test_cubes: String,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);
    let games = input.parse_lines(Game::from_str).collect::<Vec<Game>>();

    let cubes = args.test_cubes.parse::<Grab>()?;

    let mut sum = 0;
    let mut power = 0;
    for game in games {
        power += game.power();
        if game.test(&cubes) {
            sum += game.id();
        }
    }
    println!("The sum of the valid game ID's is {}", sum);
    println!("The total power of all games is {}", power);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = Game> {
        InputReader::from("./src/day02/test.txt").parse_lines(Game::from_str)
    }

    #[test]
    fn test_game_test() {
        let games: Vec<Game> = test_data().collect();
        let cubes = "12 red, 13 green, 14 blue".parse().unwrap();

        assert!(games[0].test(&cubes));
        assert!(games[1].test(&cubes));
        assert!(!games[2].test(&cubes));
        assert!(!games[3].test(&cubes));
        assert!(games[4].test(&cubes));
    }

    #[test]
    fn test_game_power() {
        let games: Vec<Game> = test_data().collect();

        assert_eq!(games[0].power(), 48);
        assert_eq!(games[1].power(), 12);
        assert_eq!(games[2].power(), 1560);
        assert_eq!(games[3].power(), 630);
        assert_eq!(games[4].power(), 36);
    }
}
