//! The solution for [advent of code 2023, day 22](https://adventofcode.com/2023/day/22)

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::brick::Brick;

mod brick;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 22")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let mut bricks = InputReader::from(args.input)
        .parse_lines(Brick::from_str)
        .collect::<Vec<_>>();

    bricks = sort_bricks(bricks);

    settle(&mut bricks);
    let support_structure = find_support_structure(&bricks);
    let safe_to_disintegrate =
        find_bricks_that_are_safe_to_disintegrate(&bricks, &support_structure);

    println!(
        "After settling once, there are {} bricks that can safely be disintegrated",
        safe_to_disintegrate.len()
    );

    println!(
        "The sum of the counts of is {}",
        compute_sum_of_falling_bricks_after_disintegrating(&bricks, &support_structure)
    );
}

fn sort_bricks(mut bricks: Vec<Brick>) -> Vec<Brick> {
    bricks.sort_by_key(|brick| brick.get_z().0);
    bricks
}

fn settle(bricks: &mut Vec<Brick>) {
    for i in 0..bricks.len() {
        let mut z = 0;
        for other in bricks.iter().take(i).rev() {
            if bricks[i].supported_by(other) {
                z = z.max(other.get_z().1 + 1);
            }
        }
        bricks[i].set_z(z);
    }
}

type SupportStructure = HashMap<usize, HashSet<usize>>;
fn find_support_structure(bricks: &[Brick]) -> SupportStructure {
    let mut support_structure: SupportStructure = bricks
        .iter()
        .enumerate()
        .map(|(i, _)| (i, HashSet::new()))
        .collect();

    for (i, b1) in bricks.iter().enumerate() {
        let z = b1.get_z().0;
        let bricks_below = bricks
            .iter()
            .enumerate()
            .take(i)
            .rev()
            .filter(|(_, b2)| b2.get_z().1 + 1 == z);

        for (j, b2) in bricks_below {
            if b1.supported_by(b2) {
                // println!("brick #{i} ({b1}) is supported by brick #{j} (b2)");
                support_structure.entry(i).or_default().insert(j);
            }
        }
    }

    support_structure
}

type SafeToDisintegrate = HashSet<usize>;
fn find_bricks_that_are_safe_to_disintegrate(
    bricks: &[Brick],
    support_structure: &SupportStructure,
) -> SafeToDisintegrate {
    let mut safe_to_disintegrate: SafeToDisintegrate = (0..bricks.len()).collect();
    for supports in support_structure.values() {
        if supports.len() == 1 {
            let idx = supports.iter().next().unwrap();
            safe_to_disintegrate.remove(idx);
        }
    }
    safe_to_disintegrate
}

fn compute_sum_of_falling_bricks_after_disintegrating(
    bricks: &[Brick],
    support_structure: &SupportStructure,
) -> usize {
    let mut sum = 0;
    for brick_idx in 0..bricks.len() {
        let mut removed = HashSet::from([brick_idx]);
        loop {
            let mut done = true;
            for (idx, brick) in bricks.iter().enumerate() {
                if removed.contains(&idx) || brick.get_z().0 == 0 {
                    continue;
                }
                let mut supports = support_structure.get(&idx).unwrap().clone();
                for i in removed.iter() {
                    supports.remove(i);
                }
                if supports.is_empty() {
                    removed.insert(idx);
                    done = false;
                }
            }
            if done {
                break;
            }
        }
        sum += removed.len() - 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<Brick> {
        InputReader::from("./src/day22/test.txt")
            .parse_lines(Brick::from_str)
            .collect()
    }

    #[test]
    fn test_disintegration() {
        let mut bricks = test_data();
        settle(&mut bricks);
        let support_structure = find_support_structure(&bricks);
        let safe_to_disintegrate =
            find_bricks_that_are_safe_to_disintegrate(&bricks, &support_structure);

        assert_eq!(safe_to_disintegrate, HashSet::from([1, 2, 3, 4, 6]));

        let sum = compute_sum_of_falling_bricks_after_disintegrating(&bricks, &support_structure);
        assert_eq!(sum, 7);
    }
}
