use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::parse_error;

fn main() {
    let args = get_args(&["<input file>", "<scoring round>", "<scoring offset>"], 1);
    let (nr_players, last_marble) = parse_input(&args[1]).or_exit_with(1);
    let scoring_round = args[2].parse().or_exit_with(1);
    let scoring_offset = args[3].parse().or_exit_with(1);

    println!(
        "The high score is {}.",
        play_game(nr_players, last_marble, scoring_round, scoring_offset)
    );

    println!(
        "If the last marble were {}, then the high score would be {}.",
        last_marble * 100,
        play_game(nr_players, last_marble * 100, scoring_round, scoring_offset)
    );
}

fn play_game(
    nr_players: usize,
    last_marble: usize,
    scoring_round: usize,
    scoring_offset: usize,
) -> usize {
    let mut scores = vec![0; nr_players];
    let mut circle = vec![usize::MAX; last_marble + 1];
    let mut circle_rev = vec![usize::MAX; last_marble + 1];
    circle[0] = 0;
    circle_rev[0] = 0;

    let mut current = 0;
    let mut next = 1;
    let mut player = 1;
    while next <= last_marble {
        if next % scoring_round == 0 {
            scores[player - 1] += next;
            let mut to_remove = current;
            for _ in 0..scoring_offset {
                to_remove = circle_rev[to_remove];
            }
            scores[player - 1] += to_remove;

            let after = circle[to_remove];
            let before = circle_rev[to_remove];

            circle[before] = after;
            circle_rev[after] = before;

            current = after;
        } else {
            let a = circle[current];
            let b = circle[a];

            circle[a] = next;
            circle[next] = b;

            circle_rev[b] = next;
            circle_rev[next] = a;

            current = next;
        }

        next += 1;
        player = player % nr_players + 1;
    }

    *scores.iter().max().unwrap()
}

fn parse_input(path: &str) -> Result<(usize, usize), ParseError> {
    let file = File::open(path)?;
    if let Some(Ok(line)) = BufReader::new(file).lines().next() {
        let nr_players = if let Some(idx) = line.find(" players") {
            line[..idx].parse::<usize>()?
        } else {
            return Err(parse_error!("Could not parse number of players."));
        };

        let points_bounds_opt = (line.find("worth ").map(|idx| idx + 6), line.rfind(' '));
        let last_marble = if let (Some(start_idx), Some(end_idx)) = points_bounds_opt {
            line[start_idx..end_idx].parse::<usize>()?
        } else {
            return Err(parse_error!("Could not parse number of players."));
        };

        Ok((nr_players, last_marble))
    } else {
        Err(parse_error!("Invalid input file"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(play_game(9, 25, 23, 7), 32);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(play_game(10, 1618, 23, 7), 8317);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(play_game(13, 7999, 23, 7), 146373);
    }

    #[test]
    fn test_example_4() {
        assert_eq!(play_game(17, 1104, 23, 7), 2764);
    }

    #[test]
    fn test_example_5() {
        assert_eq!(play_game(21, 6111, 23, 7), 54718);
    }

    #[test]
    fn test_example_6() {
        assert_eq!(play_game(30, 5807, 23, 7), 37305);
    }
}
