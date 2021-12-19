extern crate rdcl_aoc_helpers;

use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

fn main() {
    let args = get_args(&["<input file>", "<window>"], 1);

    let values = File::open(&args[1]).read_lines(1).collect::<Vec<i32>>();
    let window = args[2].parse::<usize>().or_exit_with(1);

    println!("{}", count_increases(&values, window));
}

fn count_increases(values: &[i32], window_size: usize) -> i32 {
    if values.len() < window_size {
        return 0;
    }

    let mut values_iter = values.iter();
    let mut count = 0;
    let mut window = values_iter
        .by_ref()
        .take(window_size)
        .copied()
        .collect::<Vec<i32>>();
    let mut index = 0;
    let mut previous_sum: i32 = window.iter().sum();

    for &val in values_iter {
        let sum = previous_sum - window[index] + val;
        if sum > previous_sum {
            count += 1;
        }
        window[index] = val;
        index = (index + 1) % window_size;
        previous_sum = sum;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_increases_with_window_1() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increases(&input, 1), 7);
    }

    #[test]
    fn test_count_increases_with_window_3() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increases(&input, 3), 5);
    }
}
