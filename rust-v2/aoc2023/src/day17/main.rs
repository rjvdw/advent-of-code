//! The solution for [advent of code 2023, day 17](https://adventofcode.com/2023/day/17)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::city::City;

mod city;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 17")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let mut city = InputReader::from(args.input).parse::<City>();

    match city.find_optimal_route() {
        Some(value) => {
            println!("The optimal route through the city incurs a heat loss of {value}");
        }
        None => {
            eprintln!("There exists no route through the city");
        }
    }

    println!("Upgrading to ultra crucibles...");
    city.upgrade_to_ultra_crucibles();

    match city.find_optimal_route() {
        Some(value) => {
            println!("The optimal route through the city incurs a heat loss of {value}");
        }
        None => {
            eprintln!("There exists no route through the city");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> City {
        InputReader::from("./src/day17/test.txt").parse()
    }

    #[test]
    fn test_find_optimal_route() {
        assert_eq!(test_data().find_optimal_route(), Some(102));
    }

    #[test]
    fn test_find_optimal_route_with_ultra_crucibles() {
        let mut city = test_data();
        city.upgrade_to_ultra_crucibles();
        assert_eq!(city.find_optimal_route(), Some(94));
    }
}
