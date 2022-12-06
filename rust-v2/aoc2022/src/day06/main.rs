//! The solution for [advent of code 2022, day 6](https://adventofcode.com/2022/day/6)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 6")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The size of the marker.
    #[clap(short, long, value_parser, default_value_t = 4)]
    size: usize,
}

fn main() {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input).read_line();

    match find_start_index(&input, args.size) {
        Some(p) => println!("The input starts at position {}", p),
        None => eprintln!("The start-of-packet marker could not be found"),
    }
}

fn find_start_index(input: &str, size: usize) -> Option<usize> {
    input
        .len()
        .checked_sub(size)
        .and_then(|max| (0..max).find(|&i| is_distinct(&input[i..(i + size)])))
        .map(|i| i + size)
}

fn is_distinct(slice: &str) -> bool {
    slice
        .chars()
        .enumerate()
        .all(|(i, v1)| slice.chars().skip(i + 1).all(|v2| v1 != v2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const TEST_DATA_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const TEST_DATA_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const TEST_DATA_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const TEST_DATA_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_find_start_index_with_size_4() {
        assert_eq!(find_start_index(TEST_DATA_1, 4), Some(7));
        assert_eq!(find_start_index(TEST_DATA_2, 4), Some(5));
        assert_eq!(find_start_index(TEST_DATA_3, 4), Some(6));
        assert_eq!(find_start_index(TEST_DATA_4, 4), Some(10));
        assert_eq!(find_start_index(TEST_DATA_5, 4), Some(11));
    }

    #[test]
    fn test_find_start_index_with_size_14() {
        assert_eq!(find_start_index(TEST_DATA_1, 14), Some(19));
        assert_eq!(find_start_index(TEST_DATA_2, 14), Some(23));
        assert_eq!(find_start_index(TEST_DATA_3, 14), Some(23));
        assert_eq!(find_start_index(TEST_DATA_4, 14), Some(29));
        assert_eq!(find_start_index(TEST_DATA_5, 14), Some(26));
    }
}
