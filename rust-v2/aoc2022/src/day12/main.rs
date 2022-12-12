//! The solution for [advent of code 2022, day 12](https://adventofcode.com/2022/day/12)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::MainResult;

use crate::heightmap::Heightmap;

mod heightmap;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 12")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);
    let heightmap = Heightmap::parse(input.read_lines())?;

    let shortest_path = heightmap.find_shortest_path();
    println!("The shortest path has length {}", shortest_path.len() - 1);

    let (starting_point, shortest_path) =
        heightmap.find_shortest_path_with_alternative_starting_point();
    println!(
        "If you start at {:?}, the shortest path has length {}",
        starting_point,
        shortest_path.len() - 1
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = String> {
        InputReader::from("./src/day12/test.txt").read_lines()
    }

    #[test]
    fn test_find_shortest_path() {
        let heightmap = Heightmap::parse(test_data()).unwrap();
        assert_eq!(heightmap.find_shortest_path().len(), 32);
    }

    #[test]
    fn test_find_shortest_path_with_alternative_starting_point() {
        let heightmap = Heightmap::parse(test_data()).unwrap();
        let (starting_point, shortest_path) =
            heightmap.find_shortest_path_with_alternative_starting_point();

        assert_eq!(starting_point, (0, 4));
        assert_eq!(shortest_path.len(), 30);
    }
}
