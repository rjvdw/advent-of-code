#[macro_use]
extern crate lazy_static;
extern crate rdcl_aoc_helpers;

use std::collections::HashMap;
use std::fs::File;
use std::sync::Mutex;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

const GESTATION_PERIOD: usize = 9;
const REPRODUCTION_RATE: usize = 7;

lazy_static! {
    static ref CACHE: Mutex<HashMap<usize, usize>> = Mutex::new(HashMap::new());
}

fn main() {
    let args = get_args(&["<input file>", "<days>"], 1);

    let values = File::open(&args[1])
        .read_lines::<String>(1)
        .next()
        .or_exit_with(1)
        .split(',')
        .map(|x| x.parse::<usize>().or_exit_with(1))
        .collect::<Vec<usize>>();
    let days = args[2].parse::<usize>().or_exit_with(1);

    println!(
        "After {} days, there are {} lantern fish.",
        days,
        solve(&values, days),
    );
}

fn solve(values: &[usize], n: usize) -> usize {
    values
        .iter()
        .map(|v| fibo_alt(n + GESTATION_PERIOD - v - 1))
        .sum()
}

fn fibo_alt(n: usize) -> usize {
    let opt = CACHE.lock().unwrap().get(&n).copied();
    if let Some(v) = opt {
        v
    } else {
        let v = if n < GESTATION_PERIOD {
            1
        } else {
            fibo_alt(n - GESTATION_PERIOD) + fibo_alt(n - REPRODUCTION_RATE)
        };
        CACHE.lock().unwrap().insert(n, v);
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let values = vec![3, 4, 3, 1, 2];
        assert_eq!(solve(&values, 0), 5);
        assert_eq!(solve(&values, 1), 5);
        assert_eq!(solve(&values, 2), 6);
        assert_eq!(solve(&values, 3), 7);
        assert_eq!(solve(&values, 4), 9);
        assert_eq!(solve(&values, 5), 10);
        assert_eq!(solve(&values, 80), 5934);
        assert_eq!(solve(&values, 256), 26984457539);
    }
}
