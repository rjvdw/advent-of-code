extern crate helpers;

use std::cmp::Ordering;
use std::collections::VecDeque;
use std::env;
use std::process::exit;

use helpers::{handle_result, read_input};

/// https://adventofcode.com/2020/day/9
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input file> <preamble size>", &args[0]);
        exit(1);
    }

    let path = &args[1];
    let values: Vec<u64> = handle_result(read_input(path));
    let preamble_size = handle_result(args[2].parse::<usize>());

    match find_first_invalid_number(&values, preamble_size) {
        Some((v, l)) => {
            println!("First invalid number on line {}: {}", l + 1, v);

            match find_contiguous_numbers_that_sum_to(&values, v) {
                Some((start, end)) => {
                    println!(
                        "Found a set of contiguous numbers, starting at line {}, and ending at line {}.",
                        start + 1,
                        end + 1
                    );
                    let sum = get_sum_of_smallest_and_largest_values_from(&values, start, end);
                    println!(
                        "The sum of the smallest and the largest number in this range is: {}",
                        sum
                    );
                }
                None => println!("No set of contiguous numbers sum to this value."),
            };
        }
        None => println!("No invalid numbers."),
    };
}

fn find_first_invalid_number(values: &[u64], preamble_size: usize) -> Option<(u64, usize)> {
    let mut current_slice: VecDeque<u64> = VecDeque::with_capacity(preamble_size);
    for &value in values.iter().take(preamble_size) {
        current_slice.push_back(value);
    }

    let mut idx = preamble_size;
    while idx < values.len() {
        let value = values[idx];
        if !is_valid_number(&current_slice, value) {
            return Some((value, idx));
        }

        idx += 1;
        current_slice.pop_front();
        current_slice.push_back(value);
    }

    None
}

fn is_valid_number(values: &VecDeque<u64>, next: u64) -> bool {
    for (i, x) in values.iter().enumerate() {
        for y in values.iter().skip(i + 1) {
            if x + y == next {
                return true;
            }
        }
    }

    false
}

fn find_contiguous_numbers_that_sum_to(values: &[u64], target: u64) -> Option<(usize, usize)> {
    for (i, _) in values.iter().enumerate() {
        let mut sum = 0;
        for (j, y) in values.iter().skip(i).enumerate() {
            let j = i + j;
            sum += y;
            match sum.cmp(&target) {
                Ordering::Equal => return Some((i, j)),
                Ordering::Greater => break,
                _ => {}
            }
        }
    }

    None
}

fn get_sum_of_smallest_and_largest_values_from(values: &[u64], start: usize, end: usize) -> u64 {
    let mut min = values[start];
    let mut max = values[start];

    for &value in values.iter().skip(start).take(end - start) {
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
    }

    min + max
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn test() {
            let values = vec![
                35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                309, 576,
            ];

            assert_eq!(find_first_invalid_number(&values, 5), Some((127, 14)));
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn test() {
            let values = vec![
                35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                309, 576,
            ];

            assert_eq!(
                find_contiguous_numbers_that_sum_to(&values, 127),
                Some((2, 5))
            );
            assert_eq!(
                get_sum_of_smallest_and_largest_values_from(&values, 2, 5),
                62
            );
        }
    }
}
