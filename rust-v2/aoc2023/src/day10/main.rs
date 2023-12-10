//! The solution for [advent of code 2023, day 10](https://adventofcode.com/2023/day/10)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::pipe::PipeMap;

mod pipe;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 10")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let mut map = InputReader::from(args.input).parse::<PipeMap>();

    match map.find_loop() {
        Some(steps) => {
            println!("The furthest point takes {steps} steps");
            let inside = map.count_points_inside_loop();
            println!("There are {inside} points inside the loop");
        }
        None => println!("This puzzle input contains no loops"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data(file: &str) -> PipeMap {
        InputReader::from(format!("./src/day10/{file}")).parse::<PipeMap>()
    }

    #[test]
    fn test_find_loop_1() {
        let mut map = test_data("test1.txt");
        // println!("{map}");
        assert_eq!(map.find_loop().unwrap(), 4);
        assert_eq!(map.count_points_inside_loop(), 1);
    }

    #[test]
    fn test_find_loop_2() {
        let mut map = test_data("test2.txt");
        // println!("{map}");
        assert_eq!(map.find_loop().unwrap(), 8);
        assert_eq!(map.count_points_inside_loop(), 1);
    }

    #[test]
    fn test_find_loop_3() {
        let mut map = test_data("test3.txt");
        // println!("{map}");
        assert_eq!(map.find_loop().unwrap(), 23);
        assert_eq!(map.count_points_inside_loop(), 4);
    }

    #[test]
    fn test_find_loop_4() {
        let mut map = test_data("test4.txt");
        // println!("{map}");
        assert_eq!(map.find_loop().unwrap(), 70);
        assert_eq!(map.count_points_inside_loop(), 8);
    }

    #[test]
    fn test_find_loop_5() {
        let mut map = test_data("test5.txt");
        // println!("{map}");
        assert_eq!(map.find_loop().unwrap(), 80);
        assert_eq!(map.count_points_inside_loop(), 10);
    }
}
