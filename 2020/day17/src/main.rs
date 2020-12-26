extern crate rdcl_aoc_helpers;

use std::collections::{HashMap, HashSet};
use std::env;
use std::process::exit;

use rdcl_aoc_helpers::error::WithOrExit;

use crate::cell::Cell;
use crate::parser::read;

mod cell;
mod parser;

/// https://adventofcode.com/2020/day/17
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input file> <steps>", &args[0]);
        exit(1);
    }

    let input = read(&args[1]).or_exit_with(1);
    let steps = args[2].parse::<usize>().or_exit_with(1);

    println!(
        "[3d] After {} steps, there are {} active cells.",
        steps,
        solve::<(i32, i32, i32)>(&input, steps)
    );

    println!(
        "[4d] After {} steps, there are {} active cells.",
        steps,
        solve::<(i32, i32, i32, i32)>(&input, steps)
    );
}

fn solve<T: Cell>(input: &[(i32, i32)], steps: usize) -> usize {
    let mut active: HashSet<T> = HashSet::new();

    for &(x, y) in input {
        active.insert(T::from(x, y));
    }

    for _ in 0..steps {
        let mut active_neighbours: HashMap<T, usize> = HashMap::new();

        for cell in &active {
            for neighbour in cell.neighbours() {
                *active_neighbours.entry(neighbour).or_insert(0) += 1;
            }
        }

        let mut next_active: HashSet<T> = HashSet::new();
        for (cell, count) in active_neighbours {
            if active.contains(&cell) {
                if count == 2 || count == 3 {
                    next_active.insert(cell);
                }
            } else if count == 3 {
                next_active.insert(cell);
            }
        }
        active = next_active;
    }

    active.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3d() {
        let input = vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
        assert_eq!(solve::<(i32, i32, i32)>(&input, 6), 112);
    }

    #[test]
    fn test_4d() {
        let input = vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
        assert_eq!(solve::<(i32, i32, i32, i32)>(&input, 6), 848);
    }
}
