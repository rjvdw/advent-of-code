//! The solution for [advent of code 2022, day 24](https://adventofcode.com/2022/day/24)

extern crate core;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::valley::Valley;

mod direction;
mod valley;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 24")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let valley = InputReader::from(args.input).parse::<Valley>();

    match there_and_back_again(valley) {
        Some((m1, m2, m3)) => {
            println!("The total time is {}", m1 + m2 + m3);
        }
        None => {
            eprintln!("There is no path through the valley.");
        }
    };
}

fn there_and_back_again(valley: Valley) -> Option<(usize, usize, usize)> {
    let (valley, m1) = find_shortest_path(&valley)?;
    println!("It will take {m1} minutes to pass through the valley.");
    let valley = valley.flip();
    let (valley, m2) = find_shortest_path(&valley)?;
    println!("It will take {m2} minutes to go back for the snack.");
    let valley = valley.flip();
    let (_, m3) = find_shortest_path(&valley)?;
    println!("It will take {m3} minutes to pass through the valley again.");

    Some((m1, m2, m3))
}

fn find_shortest_path(valley: &Valley) -> Option<(Valley, usize)> {
    let mut open_set: BinaryHeap<State> = BinaryHeap::new();
    open_set.push(State {
        valley: valley.clone(),
        steps: 0,
    });

    let mut seen: HashSet<String> = HashSet::new();
    seen.insert(summarize(valley));

    while let Some(State { valley, steps }) = open_set.pop() {
        let next_steps = steps + 1;
        for next_valley in valley.get_possible_transitions() {
            if next_valley.done() {
                return Some((next_valley, next_steps));
            }
            let summary = summarize(&next_valley);
            if seen.contains(&summary) {
                // this exact state has been seen before so can be ignored
                continue;
            }
            seen.insert(summary);
            open_set.push(State {
                valley: next_valley,
                steps: next_steps,
            });
        }
    }

    None
}

fn summarize(valley: &Valley) -> String {
    format!("{valley}")
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    valley: Valley,
    steps: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let d_1 = self.valley.distance_to_goal();
        let d_2 = other.valley.distance_to_goal();

        let d_min_1 = self.steps + d_1;
        let d_min_2 = other.steps + d_2;

        d_min_1.cmp(&d_min_2).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::valley::tests::test_data;

    use super::*;

    #[test]
    fn test_find_shortest_path() {
        let valley = test_data();
        let (valley, m1) = find_shortest_path(&valley).unwrap();
        assert_eq!(m1, 18);

        let valley = valley.flip();
        let (valley, m2) = find_shortest_path(&valley).unwrap();
        assert_eq!(m2, 23);

        let valley = valley.flip();
        let (_, m3) = find_shortest_path(&valley).unwrap();
        assert_eq!(m3, 13);
    }
}
