mod input_record;

extern crate helpers;

use std::env;
use std::process::exit;
use helpers::{read_input, handle_result};
use input_record::InputRecord;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input file>", &args[0]);
        exit(1);
    }

    let path = &args[1];
    let values: Vec<InputRecord> = handle_result(read_input(path));

    println!("{}", solve(&values, 3, 1));

    println!("{}", solve(&values, 1, 1)
        * solve(&values, 3, 1)
        * solve(&values, 5, 1)
        * solve(&values, 7, 1)
        * solve(&values, 1, 2)
    );
}

fn solve(values: &Vec<InputRecord>, step_x: usize, step_y: usize) -> u64 {
    let mut nr_trees = 0;
    let mut pos = 0;

    for row in values.iter().step_by(step_y) {
        if row.test_index(pos) {
            nr_trees += 1;
        }
        pos += step_x;
    }

    nr_trees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ].iter().map(|l| l.parse::<InputRecord>().unwrap()).collect();

        assert_eq!(solve(&input, 1, 1), 2);
        assert_eq!(solve(&input, 3, 1), 7);
        assert_eq!(solve(&input, 5, 1), 3);
        assert_eq!(solve(&input, 7, 1), 4);
        assert_eq!(solve(&input, 1, 2), 2);
    }
}