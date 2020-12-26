extern crate rdcl_aoc_helpers;

use std::cmp::Ordering;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::process::exit;

use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

/// https://adventofcode.com/2020/day/9
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input file> <preamble size>", &args[0]);
        exit(1);
    }

    let numbers = File::open(&args[1]).read_lines(1).collect::<Vec<u64>>();
    let preamble_size = args[2].parse::<usize>().or_exit_with(1);

    match find_first_invalid_number(&numbers, preamble_size) {
        Some((v, l)) => {
            println!("First invalid number on line {}: {}", l + 1, v);

            match find_contiguous_numbers_that_sum_to(&numbers, v) {
                Some((start, end)) => {
                    println!(
                        "Found a set of contiguous numbers, starting at line {}, and ending at line {}.",
                        start + 1,
                        end + 1
                    );
                    let sum = get_sum_of_smallest_and_largest_values_from(&numbers, start, end);
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

fn find_first_invalid_number(numbers: &[u64], preamble_size: usize) -> Option<(u64, usize)> {
    let mut current_slice: VecDeque<u64> = VecDeque::with_capacity(preamble_size);
    for &number in numbers.iter().take(preamble_size) {
        current_slice.push_back(number);
    }

    let mut idx = preamble_size;
    while idx < numbers.len() {
        let number = numbers[idx];
        if !is_valid_number(&current_slice, number) {
            return Some((number, idx));
        }

        idx += 1;
        current_slice.pop_front();
        current_slice.push_back(number);
    }

    None
}

fn is_valid_number(numbers: &VecDeque<u64>, next: u64) -> bool {
    for (i, x) in numbers.iter().enumerate() {
        for y in numbers.iter().skip(i + 1) {
            if x + y == next {
                return true;
            }
        }
    }

    false
}

fn find_contiguous_numbers_that_sum_to(numbers: &[u64], target: u64) -> Option<(usize, usize)> {
    for (i, _) in numbers.iter().enumerate() {
        let mut sum = 0;
        for (j, y) in numbers.iter().skip(i).enumerate() {
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

fn get_sum_of_smallest_and_largest_values_from(numbers: &[u64], start: usize, end: usize) -> u64 {
    let mut min = numbers[start];
    let mut max = numbers[start];

    for &number in numbers.iter().skip(start).take(end - start) {
        if number < min {
            min = number;
        }
        if number > max {
            max = number;
        }
    }

    min + max
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUMBERS: [u64; 20] = [
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    #[test]
    fn test_find_first_invalid_number() {
        assert_eq!(find_first_invalid_number(&NUMBERS, 5), Some((127, 14)));
    }

    #[test]
    fn test_find_contiguous_numbers_that_sum_to() {
        assert_eq!(
            find_contiguous_numbers_that_sum_to(&NUMBERS, 127),
            Some((2, 5))
        );
    }

    #[test]
    fn test_get_sum_of_smallest_and_largest_values_from() {
        assert_eq!(
            get_sum_of_smallest_and_largest_values_from(&NUMBERS, 2, 5),
            62
        );
    }
}
