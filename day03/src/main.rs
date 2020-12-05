extern crate helpers;

use std::env;
use std::process::exit;

use helpers::{handle_result, read_input};
use input_record::InputRecord;

mod input_record;

/// https://adventofcode.com/2020/day/3
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input file> <x1>,<y1> ... <xn>,<yn>", &args[0]);
        exit(1);
    }

    let path = &args[1];
    let values: Vec<InputRecord> = handle_result(read_input(path));

    let mut result = 1;
    for entry in args.iter().skip(2) {
        let vec = entry.split(',')
            .map(|c| handle_result(c.parse::<usize>()))
            .collect::<Vec<usize>>();

        if let [x, y] = &vec[..] {
            result *= solve(&values, *x, *y);
        } else {
            eprintln!("Invalid input: {}", entry);
            exit(1);
        }
    }

    println!("{}", result);
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
