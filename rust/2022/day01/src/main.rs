extern crate rdcl_aoc_helpers;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    let args = get_args(&["<input file>", "<top n>"], 1);

    let file = File::open(&args[1]).or_exit_with(1);
    let lines = BufReader::new(file).lines();
    let numbers = parse_input(lines).or_exit_with(1);

    let n = args[2].parse::<usize>().or_exit_with(1);

    match max_calories(&numbers, n) {
        Some(v) => println!("The sum of the largest {} values is {}", n, v),
        None => eprintln!(
            "There are insufficient values to compute the sum of the largest {} values",
            n
        ),
    }
}

fn max_calories(values: &[u32], n: usize) -> Option<u32> {
    if values.len() < n {
        None
    } else {
        let mut values = values.to_vec();
        values.sort_unstable();
        Some(values.iter().rev().take(n).sum())
    }
}

fn parse_input<T>(lines: T) -> Result<Vec<u32>, ParseError>
where
    T: Iterator<Item = io::Result<String>>,
{
    let mut sums = vec![];
    let mut sum = 0;

    for line in lines {
        let line = line?;
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<u32>()?;
        }
    }
    sums.push(sum);

    Ok(sums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = vec![
            Ok("1000".to_string()),
            Ok("2000".to_string()),
            Ok("3000".to_string()),
            Ok("".to_string()),
            Ok("4000".to_string()),
            Ok("".to_string()),
            Ok("5000".to_string()),
            Ok("6000".to_string()),
            Ok("".to_string()),
            Ok("7000".to_string()),
            Ok("8000".to_string()),
            Ok("9000".to_string()),
            Ok("".to_string()),
            Ok("10000".to_string()),
        ];
        assert_eq!(
            parse_input(input.into_iter()),
            Ok(vec![6000, 4000, 11000, 24000, 10000])
        );
    }

    #[test]
    fn test_max_calories_with_n_is_1() {
        let input = vec![6000, 4000, 11000, 24000, 10000];
        assert_eq!(max_calories(&input, 1), Some(24000));
    }

    #[test]
    fn test_max_calories_with_n_is_3() {
        let input = vec![6000, 4000, 11000, 24000, 10000];
        assert_eq!(max_calories(&input, 3), Some(24000 + 11000 + 10000));
    }

    #[test]
    fn test_max_calories_with_n_is_6() {
        let input = vec![6000, 4000, 11000, 24000, 10000];
        assert_eq!(max_calories(&input, 6), None);
    }
}
