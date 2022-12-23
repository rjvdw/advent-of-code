//! The solution for [advent of code 2022, day 23](https://adventofcode.com/2022/day/23)

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::direction::{Direction, Headings, HEADINGS};

mod direction;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 23")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The maximum number of rounds to evaluate.
    #[clap(long, value_parser, default_value_t = 1_000_000_000)]
    max_rounds: usize,
}

fn main() {
    let args: Args = Args::parse();
    let mut elves = parse(InputReader::from(args.input).read_lines());
    let mut headings = HEADINGS;

    for round in 1..=args.max_rounds {
        let next = step(&elves, headings);

        if next.0 == elves {
            println!("Round {round} is the first round where no elf moved.");
            return;
        }

        elves = next.0;
        headings = next.1;

        if round == 10 {
            println!(
                "After 10 rounds, there are {} empty ground tiles",
                count_empty(&elves)
            );
        }
    }

    eprintln!(
        "after {} rounds, there were still elves moving",
        args.max_rounds
    );
}

fn step(elves: &HashSet<(i64, i64)>, headings: Headings) -> (HashSet<(i64, i64)>, Headings) {
    // the new proposed locations and which elf wants to move there
    let mut proposal: HashMap<(i64, i64), (i64, i64)> = HashMap::new();

    // all locations that have been proposed by some elf during this round
    let mut proposed_by_other_elf: HashSet<(i64, i64)> = HashSet::new();

    for &elf in elves {
        if noone_around(elves, elf) {
            proposal.insert(elf, elf);
        } else {
            let mut did_move = false;
            for heading in headings {
                if is_clear(elves, elf, heading) {
                    let next = heading[1].from(elf);

                    if proposed_by_other_elf.contains(&next) {
                        // someone else already proposed this, stay put
                        proposal.insert(elf, elf);

                        if proposal.contains_key(&next) {
                            // the other elf that proposed this location also stays put
                            let other = *proposal.get(&next).unwrap();
                            proposal.remove(&next);
                            proposal.insert(other, other);
                        }
                    } else {
                        // no-one proposed this yet, propose moving here
                        proposal.insert(next, elf);
                        proposed_by_other_elf.insert(next);
                    }

                    did_move = true;
                    break;
                }
            }
            if !did_move {
                proposal.insert(elf, elf);
            }
        }
    }

    if proposal.len() != elves.len() {
        panic!(
            "something went wrong, the number of proposals differs from the number of elves; {:?}",
            proposal
        );
    }

    (
        proposal.into_keys().collect(),
        [headings[1], headings[2], headings[3], headings[0]],
    )
}

fn noone_around(elves: &HashSet<(i64, i64)>, elf: (i64, i64)) -> bool {
    direction::ALL.iter().all(|d| !elves.contains(&d.from(elf)))
}

fn is_clear(elves: &HashSet<(i64, i64)>, elf: (i64, i64), directions: [Direction; 3]) -> bool {
    directions.iter().all(|d| !elves.contains(&d.from(elf)))
}

fn count_empty(elves: &HashSet<(i64, i64)>) -> usize {
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    for &(x, y) in elves {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    let mut count = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if !elves.contains(&(x, y)) {
                count += 1;
            }
        }
    }
    count
}

fn parse<T>(input: T) -> HashSet<(i64, i64)>
where
    T: Iterator<Item = String>,
{
    let mut elves = HashSet::new();

    for (y, line) in input.enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                elves.insert((x as i64, y as i64));
            }
        }
    }

    elves
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> HashSet<(i64, i64)> {
        parse(InputReader::from("./src/day23/test.txt").read_lines())
    }

    #[test]
    fn test_10_steps() {
        let mut elves = test_data();
        let mut headings = HEADINGS;

        for _ in 1..=10 {
            let next = step(&elves, headings);
            elves = next.0;
            headings = next.1;
        }

        assert_eq!(count_empty(&elves), 110);
    }

    #[test]
    fn test_until_done() {
        let mut elves = test_data();
        let mut headings = HEADINGS;

        let mut steps = 0;
        loop {
            steps += 1;
            let next = step(&elves, headings);
            if next.0 == elves {
                break;
            }
            elves = next.0;
            headings = next.1;
        }

        assert_eq!(steps, 20);
    }
}
