extern crate helpers;

use std::env;
use std::process::exit;

use helpers::{handle_result, read_input};

/// https://adventofcode.com/2020/day/10
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input file> <max jolt difference>", &args[0]);
        exit(1);
    }

    let path = &args[1];
    let values: Vec<u32> = handle_result(read_input(path));
    let max_jolt_difference = handle_result(args[2].parse::<u32>());

    match solve_part_1(&values, max_jolt_difference) {
        Some(v) => println!("The solution to part 1 is: {}", v),
        None => println!("Could not find a solution for part 1"),
    };

    println!(
        "The solution to part 2 is: {}",
        solve_part_2(&values, max_jolt_difference)
    );
}

fn solve_part_1(values: &[u32], max_jolt_difference: u32) -> Option<u32> {
    let mut values = values.to_vec();
    values.sort_unstable();

    let mut diff_lower = 0;
    let mut diff_upper = 1; // always at least 1, because of the final step
    let mut prev = 0;

    for value in values {
        if value - prev == 1 {
            diff_lower += 1;
        } else if value - prev == max_jolt_difference {
            diff_upper += 1;
        } else if value - prev > max_jolt_difference {
            return None;
        }

        prev = value;
    }

    Some(diff_lower * diff_upper)
}

fn solve_part_2(values: &[u32], max_jolt_difference: u32) -> u64 {
    let mut values = values.to_vec();
    values.sort_unstable();
    values.push(values[values.len() - 1] + max_jolt_difference);
    let values = values;
    let result_size = max_jolt_difference as usize;

    let mut result: Vec<u64> = vec![0; result_size];
    result[(values.len() - 1) % result_size] = 1;

    for (idx, value) in values.iter().enumerate().rev().skip(1) {
        result[idx % result_size] = values
            .iter()
            .skip(idx + 1)
            .take_while(|&&v| v - value <= max_jolt_difference)
            .enumerate()
            .map(|(pos, _)| result[(pos + idx + 1) % result_size])
            .sum();
    }

    values
        .iter()
        .take_while(|&&v| v <= max_jolt_difference)
        .enumerate()
        .map(|(pos, _)| result[pos % result_size])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_1() {
        let values = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        assert_eq!(solve_part_1(&values, 3), Some(35));
    }

    #[test]
    fn test_part_1_2() {
        let values = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        assert_eq!(solve_part_1(&values, 3), Some(220));
    }

    #[test]
    fn test_part_1_3() {
        let values = vec![];

        assert_eq!(solve_part_1(&values, 3), Some(0));
    }

    #[test]
    fn test_part_1_4() {
        let values = vec![3, 9];

        assert_eq!(solve_part_1(&values, 3), None);
    }

    #[test]
    fn test_part_2_1() {
        let values = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        assert_eq!(solve_part_2(&values, 3), 8);
    }

    #[test]
    fn test_part_2_2() {
        let values = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        assert_eq!(solve_part_2(&values, 3), 19208);
    }
}
