//! The solution for [advent of code 2022, day 8](https://adventofcode.com/2022/day/8)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::forest::Forest;

mod forest;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 8")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let forest = InputReader::from(args.input).parse::<Forest>();

    println!(
        "The number of visible trees is: {}",
        forest.count_visible_trees()
    );

    println!(
        "The highest scenic score in this forest is: {}",
        forest.find_highest_scenic_score()
    );
}

#[cfg(test)]
mod tests {
    use grid::grid;

    use super::*;

    fn test_data() -> Forest {
        InputReader::from("./src/day08/test.txt").parse::<Forest>()
    }

    fn forest() -> Forest {
        Forest::new(grid![
            [3, 0, 3, 7, 3]
            [2, 5, 5, 1, 2]
            [6, 5, 3, 3, 2]
            [3, 3, 5, 4, 9]
            [3, 5, 3, 9, 0]
        ])
    }

    #[test]
    fn test_parse() {
        assert_eq!(test_data(), forest());
    }

    #[test]
    fn test_count_visible_trees() {
        let forest = forest();

        assert_eq!(forest.count_visible_trees(), 21);
    }

    #[test]
    fn test_scenic_score() {
        let forest = forest();

        // check edges
        assert_eq!(forest.scenic_score(0, 0), 0);
        assert_eq!(forest.scenic_score(0, 4), 0);
        assert_eq!(forest.scenic_score(4, 0), 0);
        assert_eq!(forest.scenic_score(4, 4), 0);

        // check interior
        assert_eq!(forest.scenic_score(1, 2), 4);
        assert_eq!(forest.scenic_score(3, 2), 8);
    }

    #[test]
    fn test_find_highest_scenic_score() {
        let forest = forest();

        assert_eq!(forest.find_highest_scenic_score(), 8);
    }
}
