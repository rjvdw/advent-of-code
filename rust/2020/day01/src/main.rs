extern crate rdcl_aoc_helpers;

use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

fn main() {
    let args = get_args(&["<input file>", "<count>", "<sum>"], 1);

    let values = File::open(&args[1]).read_lines(1).collect::<Vec<i32>>();
    let count = args[2].parse::<usize>().or_exit_with(1);
    let sum = args[3].parse::<i32>().or_exit_with(1);

    match solve(&values, 0, count, sum) {
        Some(v) => println!("{}", v),
        None => println!("No solution"),
    }
}

/// Solves the problem: Does there exist a subset of `count` numbers in `values`, such that they sum
/// to `sum`. If so, return the product of these numbers.
fn solve(values: &[i32], skip: usize, count: usize, sum: i32) -> Option<i32> {
    assert_ne!(count, 0);
    assert!(sum > 0);

    if count == 1 {
        if let Some(&v) = values.iter().skip(skip).find(|&&v| v == sum) {
            return Some(v);
        }
    } else {
        for (pos, &v) in values.iter().skip(skip).filter(|&&v| v < sum).enumerate() {
            if let Some(r) = solve(values, pos + 1, count - 1, sum - v) {
                return Some(r * v);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(solve(&input, 0, 2, 2020), Some(514579));
    }

    #[test]
    fn test_part_2() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(solve(&input, 0, 3, 2020), Some(241861950));
    }
}
