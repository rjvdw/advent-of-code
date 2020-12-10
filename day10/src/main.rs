extern crate helpers;

use std::collections::HashMap;
use std::env;
use std::process::exit;

use helpers::{handle_result, read_input};

/// https://adventofcode.com/2020/day/10
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input file>", &args[0]);
        exit(1);
    }

    let path = &args[1];
    let values: Vec<u32> = handle_result(read_input(path));

    match solve_part_1(&values) {
        Some(v) => println!("The solution to part 1 is: {}", v),
        None => println!("Could not find a solution for part 1"),
    };

    println!("The solution to part 2 is: {}", solve_part_2(&values));
}

fn solve_part_1(values: &[u32]) -> Option<u32> {
    let mut values = values.to_vec();
    values.sort_unstable();

    let mut diff_1 = 0;
    let mut diff_3 = 1; // always at least 1, because of the final step
    let mut prev = 0;

    for &value in values.iter() {
        if value - prev == 1 {
            diff_1 += 1;
        } else if value - prev == 3 {
            diff_3 += 1;
        } else if value - prev > 3 {
            return None;
        }

        prev = value;
    }

    Some(diff_1 * diff_3)
}

fn solve_part_2(values: &[u32]) -> u64 {
    let mut values = values.to_vec();
    values.sort_unstable();
    let mut cache: HashMap<usize, u64> = HashMap::new();
    match values.last() {
        Some(v) => solve_part_2_rec(&values, values.len(), 0, 0, v + 3, &mut cache),
        None => 0,
    }
}

fn solve_part_2_rec(
    values: &[u32],
    len: usize,
    offset: usize,
    prev: u32,
    target: u32,
    cache: &mut HashMap<usize, u64>,
) -> u64 {
    match cache.get(&offset) {
        Some(&sum) => sum,
        None => {
            let upper_bound = prev + 4;
            if offset < len {
                let sum = values
                    .iter()
                    .skip(offset)
                    .take_while(|&&value| value < upper_bound)
                    .enumerate()
                    .map(|(idx, &value)| {
                        solve_part_2_rec(values, len, offset + idx + 1, value, target, cache)
                    })
                    .sum();
                cache.insert(offset, sum);
                sum
            } else if target < upper_bound {
                1
            } else {
                0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_1() {
        let values = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        assert_eq!(solve_part_1(&values), Some(35));
    }

    #[test]
    fn test_part_1_2() {
        let values = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        assert_eq!(solve_part_1(&values), Some(220));
    }

    #[test]
    fn test_part_1_3() {
        let values = vec![];

        assert_eq!(solve_part_1(&values), Some(0));
    }

    #[test]
    fn test_part_1_4() {
        let values = vec![3, 9];

        assert_eq!(solve_part_1(&values), None);
    }

    #[test]
    fn test_part_2_1() {
        let values = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        assert_eq!(solve_part_2(&values), 8);
    }

    #[test]
    fn test_part_2_2() {
        let values = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        assert_eq!(solve_part_2(&values), 19208);
    }
}
