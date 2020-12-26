extern crate rdcl_aoc_helpers;

use std::fs::File;
use std::process::exit;

use rdcl_aoc_helpers::args::get_args_repeating;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

use map_row::MapRow;

mod map_row;

/// https://adventofcode.com/2020/day/3
fn main() {
    let args = get_args_repeating(&["<input file>", "<x>,<y> ... <xn>,<yn>"], 1);

    let rows = File::open(&args[1]).read_lines(1).collect::<Vec<MapRow>>();

    let mut result = 1;
    for entry in args.iter().skip(2) {
        let vec = entry
            .split(',')
            .map(|c| c.parse::<usize>().or_exit_with(1))
            .collect::<Vec<usize>>();

        if let [x, y] = &vec[..] {
            result *= count_trees_in_path(&rows, *x, *y);
        } else {
            eprintln!("Invalid input: {}", entry);
            exit(1);
        }
    }

    println!("{}", result);
}

fn count_trees_in_path(values: &[MapRow], step_x: usize, step_y: usize) -> u64 {
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
    use rdcl_aoc_helpers::input::WithAsRecords;

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
        ]
        .as_records::<MapRow>()
        .unwrap();

        assert_eq!(count_trees_in_path(&input, 1, 1), 2);
        assert_eq!(count_trees_in_path(&input, 3, 1), 7);
        assert_eq!(count_trees_in_path(&input, 5, 1), 3);
        assert_eq!(count_trees_in_path(&input, 7, 1), 4);
        assert_eq!(count_trees_in_path(&input, 1, 2), 2);
    }
}
