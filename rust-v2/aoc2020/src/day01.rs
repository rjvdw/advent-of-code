//! The solution for [advent of code 2020, day 1](https://adventofcode.com/2020/day/1)

use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::MainResult;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2020, day 1")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The amount of numbers to sum to get to the total.
    #[clap(long, short, value_parser, default_value_t = 2)]
    count: usize,

    /// The sum total to look for.
    #[clap(long, short, value_parser, default_value_t = 2020)]
    target: u32,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input)
        .parse_lines(u32::from_str)
        .collect::<Vec<u32>>();

    let result = find_numbers_that_sum_to(&input, args.target, args.count);

    match result {
        Some(numbers) => {
            let product = numbers.iter().product::<u32>();

            println!(
                "The numbers {:?} sum to {}, their product is {}",
                numbers, args.target, product
            );
        }
        None => {
            eprintln!(
                "No set of {} numbers could be found that sum to {}",
                args.count, args.target
            );
        }
    }

    Ok(())
}

fn find_numbers_that_sum_to(numbers: &[u32], target: u32, count: usize) -> Option<Vec<u32>> {
    if count == 1 {
        if numbers.contains(&target) {
            return Some(vec![target]);
        }
    } else {
        for (i, &nr) in numbers.iter().enumerate() {
            if nr < target {
                let result = find_numbers_that_sum_to(&numbers[(i + 1)..], target - nr, count - 1);
                if let Some(mut res) = result {
                    res.push(nr);
                    return Some(res);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<u32> {
        InputReader::from(PathBuf::from("./test-inputs/day01.txt"))
            .parse_lines(u32::from_str)
            .collect()
    }

    #[test]
    fn test_find_2_numbers_that_sum_to_2020() {
        let mut result = find_numbers_that_sum_to(&test_data(), 2020, 2).unwrap();
        result.sort_unstable();
        assert_eq!(result, vec![299, 1721]);
    }

    #[test]
    fn test_find_3_numbers_that_sum_to_2020() {
        let mut result = find_numbers_that_sum_to(&test_data(), 2020, 3).unwrap();
        result.sort_unstable();
        assert_eq!(result, vec![366, 675, 979]);
    }
}
