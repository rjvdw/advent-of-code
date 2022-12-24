//! The solution for [advent of code 2022, day 19](https://adventofcode.com/2022/day/19)

use std::cmp::Ordering;
use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::blueprint::Blueprint;
use crate::pool::Pool;

mod blueprint;
mod pool;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 19")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The number of minutes to run the robots.
    #[clap(short, long, value_parser, default_value_t = 24)]
    minutes: u32,
}

fn main() {
    let args: Args = Args::parse();
    let blueprints = InputReader::from(args.input)
        .parse_lines(Blueprint::from_str)
        .collect::<Vec<Blueprint>>();

    println!(
        "The sum of the quality levels is {}.",
        blueprints
            .iter()
            .map(|blueprint| eval(blueprint, args.minutes))
            .sum::<u32>()
    );
}

fn eval(blueprint: &Blueprint, minutes: u32) -> u32 {
    let initial_state = State {
        pool: Pool::default(),
        time_left: minutes,
    };

    // let mut open_set: BinaryHeap<State> = BinaryHeap::new();
    let mut open_set: Vec<State> = Vec::new();
    open_set.push(initial_state);

    let mut best = 0;
    while let Some(state) = open_set.pop() {
        let State { pool, time_left } = state;
        if time_left == 0 {
            best = best.max(pool.resources.geode);
            continue;
        }

        let potential = pool.potential(time_left);
        if potential == 0 || potential <= best {
            continue;
        }

        for pool in pool.next_pools(blueprint) {
            let next_state = State {
                pool,
                time_left: time_left - 1,
            };
            open_set.push(next_state);
        }
    }

    println!(
        "Blueprint #{} is able to produce {best} geodes, giving it a quality score of {}.",
        blueprint.id,
        best * blueprint.id
    );

    best * blueprint.id
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    pool: Pool,
    time_left: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let p1 = self.pool.potential(self.time_left);
        let p2 = other.pool.potential(other.time_left);

        p1.cmp(&p2)
            .then(
                self.pool
                    .resources
                    .obsidian
                    .cmp(&other.pool.resources.obsidian),
            )
            .then(self.pool.resources.clay.cmp(&other.pool.resources.clay))
            .reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
