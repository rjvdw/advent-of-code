#[macro_use]
extern crate lazy_static;
extern crate rdcl_aoc_helpers;

use std::collections::HashMap;
use std::fs::File;
use std::sync::Mutex;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;
use rdcl_aoc_helpers::math::abs_diff;

lazy_static! {
    static ref CACHE: Mutex<HashMap<u32, u32>> = Mutex::new(HashMap::new());
}

/// https://adventofcode.com/2021/day/7
fn main() {
    let args = get_args(&["<input file>"], 1);

    let points = File::open(&args[1])
        .read_lines::<String>(1)
        .next()
        .or_exit_with(1)
        .split(',')
        .map(|x| x.parse::<u32>().or_exit_with(1))
        .collect::<Vec<u32>>();

    let (optimal_point, fuel_cost) =
        find_optimal_point(&points, &compute_fuel_cost_naive).or_exit_with(1);
    println!(
        "Using the naive fuel computation, the optimal point is {}, with a cost of {}.",
        optimal_point, fuel_cost
    );

    let (optimal_point, fuel_cost) =
        find_optimal_point(&points, &compute_fuel_cost_correct).or_exit_with(1);
    println!(
        "Using the correct fuel computation, the optimal point is {}, with a cost of {}.",
        optimal_point, fuel_cost
    );
}

fn find_optimal_point<F>(points: &[u32], compute_fuel_cost: &F) -> Option<(u32, u32)>
where
    F: Copy + FnOnce(&[u32], u32) -> u32,
{
    let mut min = u32::MAX;
    let mut max = u32::MIN;
    for &point in points {
        if point < min {
            min = point;
        }
        if point > max {
            max = point;
        }
    }

    let mut optimum: Option<(u32, u32)> = None;
    for point in min..=max {
        let fuel_cost = compute_fuel_cost(points, point);
        match optimum {
            None => optimum = Some((point, fuel_cost)),
            Some((_, c)) if c > fuel_cost => optimum = Some((point, fuel_cost)),
            _ => {}
        }
    }

    optimum
}

fn compute_fuel_cost_naive(points: &[u32], point: u32) -> u32 {
    points.iter().map(|&p| abs_diff(p, point)).sum()
}

fn compute_fuel_cost_correct(points: &[u32], point: u32) -> u32 {
    points.iter().map(|&p| fuel_cost(abs_diff(p, point))).sum()
}

fn fuel_cost(distance: u32) -> u32 {
    *CACHE
        .lock()
        .unwrap()
        .entry(distance)
        .or_insert_with(|| (0..=distance).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_fuel_cost_naive() {
        let values = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(compute_fuel_cost_naive(&values, 2), 37);
    }

    #[test]
    fn test_compute_fuel_cost_correct() {
        let values = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(compute_fuel_cost_correct(&values, 2), 206);
        assert_eq!(compute_fuel_cost_correct(&values, 5), 168);
    }

    #[test]
    fn test_find_optimal_point() {
        let values = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(
            find_optimal_point(&values, &compute_fuel_cost_naive),
            Some((2, 37))
        );
        assert_eq!(
            find_optimal_point(&values, &compute_fuel_cost_correct),
            Some((5, 168))
        );
    }
}
