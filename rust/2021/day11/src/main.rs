extern crate rdcl_aoc_helpers;

use std::collections::HashSet;

use grid::Grid;
use itertools::Itertools;
use rdcl_aoc_helpers::args::get_args_repeating;
use rdcl_aoc_helpers::error::WithOrExit;

use shared::numeric_grid;

const MAX_RUNS: usize = 10000;

/// https://adventofcode.com/2021/day/11
fn main() {
    let args = get_args_repeating(&["<input file> ?<steps>"], 1);
    let octopuses = numeric_grid::read(&args[1]).or_exit_with(1);

    if args.len() > 2 {
        let steps = args[2].parse::<usize>().or_exit_with(1);
        println!(
            "After {} steps, there have been {} flashes.",
            steps,
            run_simulation(octopuses, steps),
        )
    } else {
        let (rows, cols) = octopuses.size();
        let nr_octopuses = rows * cols;
        let steps = run_simulation_until(octopuses, |flashes| flashes == nr_octopuses);
        println!(
            "After {} steps, all octopuses flash at the same time.",
            steps,
        )
    }
}

fn run_simulation(octopuses: Grid<u8>, steps: usize) -> usize {
    let mut total_flashes = 0;

    let mut current = octopuses;
    for _ in 0..steps {
        let (next, flashes) = tick(&current);
        total_flashes += flashes;
        current = next;
    }

    total_flashes
}

fn run_simulation_until<P>(octopuses: Grid<u8>, condition: P) -> usize
where
    P: Copy + FnOnce(usize) -> bool,
{
    let mut current = octopuses;
    let mut counter = 0;

    loop {
        counter += 1;
        let (next, flashes) = tick(&current);
        if condition(flashes) {
            return counter;
        }
        current = next;

        if counter > MAX_RUNS {
            panic!(
                "The simulation has been running for {} steps. This does not seem good.",
                counter
            );
        }
    }
}

fn tick(octopuses: &Grid<u8>) -> (Grid<u8>, usize) {
    let (rows, cols) = octopuses.size();

    // this includes the point itself, but since a point can only flash once, this is not a problem
    let neighbours = |row, col| {
        (row..row + 3)
            .cartesian_product(col..col + 3)
            .filter(|&(r, c)| r > 0 && c > 0 && r - 1 < rows && c - 1 < cols)
            .map(|(r, c)| (r - 1, c - 1))
    };

    let mut next = Grid::new(rows, cols);
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();
    let mut going_to_flash: Vec<(usize, usize)> = vec![];

    for row in 0..rows {
        for col in 0..cols {
            next[row][col] = octopuses[row][col] + 1;
            if next[row][col] > 9 {
                going_to_flash.push((row, col));
            }
        }
    }

    while let Some((row, col)) = going_to_flash.pop() {
        if !flashed.contains(&(row, col)) {
            flashed.insert((row, col));
            for (r, c) in neighbours(row, col) {
                next[r][c] += 1;
                if next[r][c] > 9 {
                    going_to_flash.push((r, c));
                }
            }
        }
    }

    let count = flashed.len();
    for (row, col) in flashed {
        next[row][col] = 0;
    }

    (next, count)
}

#[cfg(test)]
mod tests {
    use grid::grid;

    use super::*;

    #[test]
    fn test_tick_simple_1() {
        let octopuses = grid![
            [1, 1, 1, 1, 1]
            [1, 9, 9, 9, 1]
            [1, 9, 1, 9, 1]
            [1, 9, 9, 9, 1]
            [1, 1, 1, 1, 1]
        ];
        let expected = grid![
            [3, 4, 5, 4, 3]
            [4, 0, 0, 0, 4]
            [5, 0, 0, 0, 5]
            [4, 0, 0, 0, 4]
            [3, 4, 5, 4, 3]
        ];
        assert_eq!(tick(&octopuses), (expected, 9));
    }

    #[test]
    fn test_tick_simple_2() {
        let octopuses = grid![
            [3, 4, 5, 4, 3]
            [4, 0, 0, 0, 4]
            [5, 0, 0, 0, 5]
            [4, 0, 0, 0, 4]
            [3, 4, 5, 4, 3]
        ];
        let expected = grid![
            [4, 5, 6, 5, 4]
            [5, 1, 1, 1, 5]
            [6, 1, 1, 1, 6]
            [5, 1, 1, 1, 5]
            [4, 5, 6, 5, 4]
        ];
        assert_eq!(tick(&octopuses), (expected, 0));
    }

    #[test]
    fn test_run_simulation() {
        let octopuses = get_test_data();
        assert_eq!(run_simulation(octopuses, 100), 1656);
    }

    #[test]
    fn test_run_simulation_until() {
        let octopuses = get_test_data();
        assert_eq!(run_simulation_until(octopuses, |f| f == 100), 195);
    }

    fn get_test_data() -> Grid<u8> {
        let lines = vec![
            Ok("5483143223".to_string()),
            Ok("2745854711".to_string()),
            Ok("5264556173".to_string()),
            Ok("6141336146".to_string()),
            Ok("6357385478".to_string()),
            Ok("4167524645".to_string()),
            Ok("2176841721".to_string()),
            Ok("6882881134".to_string()),
            Ok("4846848554".to_string()),
            Ok("5283751526".to_string()),
        ];
        numeric_grid::parse(lines.into_iter()).unwrap()
    }
}
