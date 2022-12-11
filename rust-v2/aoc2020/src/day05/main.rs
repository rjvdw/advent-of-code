//! The solution for [advent of code 2020, day 5](https://adventofcode.com/2020/day/5)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2020, day 5")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);

    let ids = input
        .read_lines()
        .map(|s| compute_seat_id(&s))
        .collect::<Vec<u32>>();

    println!("The highest seat ID is {}", ids.iter().max().unwrap());
    match find_missing_seat_id(&ids) {
        Some(id) => println!("The missing seat ID is {}", id),
        None => eprintln!("There are nog missing seat IDs."),
    }
}

fn compute_seat_id(boarding_pass: &str) -> u32 {
    boarding_pass.chars().fold(0, |acc, ch| {
        (match ch {
            'F' | 'L' => 0,
            'B' | 'R' => 1,
            _ => panic!("Invalid boarding pass: {}", boarding_pass),
        }) + 2 * acc
    })
}

fn find_missing_seat_id(ids: &[u32]) -> Option<u32> {
    let mut ids = ids.to_vec();
    ids.sort_unstable();
    ids.first().and_then(|first| {
        ids.iter()
            .copied()
            .enumerate()
            .find(|&(idx, val)| idx != (val - first) as usize)
            .map(|(_, val)| val - 1)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_seat_id() {
        assert_eq!(compute_seat_id("FBFBBFFRLR"), 357);
        assert_eq!(compute_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(compute_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(compute_seat_id("BBFFBBFRLL"), 820);
    }

    #[test]
    fn test_find_missing_seat_id() {
        assert_eq!(find_missing_seat_id(&[5, 3, 6, 2]), Some(4));
    }
}
