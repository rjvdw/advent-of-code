extern crate rdcl_aoc_helpers;

use std::collections::HashSet;

use grid::Grid;
use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use shared::numeric_grid;

/// https://adventofcode.com/2021/day/9
fn main() {
    let args = get_args(&["<input file>"], 1);

    let height_map = numeric_grid::read(&args[1]).or_exit_with(1);

    let low_points = find_low_points(&height_map);
    let values = low_points
        .iter()
        .map(|&(row, col)| height_map[row][col])
        .collect::<Vec<u8>>();
    println!("The sum of the risk levels is {}.", calculate_risk(&values));

    let mut basins = find_basins(&height_map, &low_points);
    basins.sort_unstable();
    let three_largest = basins.iter().rev().take(3).copied().collect::<Vec<usize>>();
    println!(
        "The three largest basins have sizes {:?}. The final answer is {}.",
        three_largest,
        three_largest.iter().copied().product::<usize>(),
    );
}

fn find_low_points(height_map: &Grid<u8>) -> Vec<(usize, usize)> {
    let mut low_points = vec![];
    for row in 0..height_map.rows() {
        for col in 0..height_map.cols() {
            let value = height_map[row][col];
            let is_low_point = neighbours(height_map, row, col)
                .iter()
                .map(|&(row, col)| height_map[row][col])
                .all(|v| v > value);
            if is_low_point {
                low_points.push((row, col));
            }
        }
    }
    low_points
}

fn neighbours(height_map: &Grid<u8>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    if row > 0 {
        neighbours.push((row - 1, col));
    }
    if col > 0 {
        neighbours.push((row, col - 1));
    }
    if row + 1 < height_map.rows() {
        neighbours.push((row + 1, col));
    }
    if col + 1 < height_map.cols() {
        neighbours.push((row, col + 1));
    }

    neighbours
}

fn calculate_risk(points: &[u8]) -> u32 {
    points.iter().map(|&p| (p as u32) + 1).sum()
}

fn find_basins(height_map: &Grid<u8>, points: &[(usize, usize)]) -> Vec<usize> {
    let mut sizes = Vec::with_capacity(points.len());
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut to_explore: Vec<(usize, usize)> = vec![];
    for &point in points {
        if seen.contains(&point) {
            panic!("There seem to be multiple low points in a single basin!");
        }
        let mut count = 0;
        seen.insert(point);
        to_explore.push(point);
        while let Some((row, col)) = to_explore.pop() {
            count += 1;
            for (row, col) in neighbours(height_map, row, col) {
                let value = height_map[row][col];
                if !seen.contains(&(row, col)) && value != 9 {
                    seen.insert((row, col));
                    to_explore.push((row, col));
                }
            }
        }
        sizes.push(count);
    }
    sizes
}

#[cfg(test)]
mod tests {
    use grid::grid;

    use super::*;

    #[test]
    fn test_find_low_points() {
        let height_map = get_test_data();
        assert_eq!(
            find_low_points(&height_map),
            vec![(0, 1), (0, 9), (2, 2), (4, 6)]
        );
    }

    #[test]
    fn test_calculate_risk() {
        let points = vec![1, 0, 5, 5];
        assert_eq!(calculate_risk(&points), 15);
    }

    #[test]
    fn test_find_basins() {
        let height_map = get_test_data();
        let low_points = find_low_points(&height_map);
        assert_eq!(find_basins(&height_map, &low_points), vec![3, 9, 14, 9]);
    }

    fn get_test_data() -> Grid<u8> {
        grid![
            [2, 1, 9, 9, 9, 4, 3, 2, 1, 0]
            [3, 9, 8, 7, 8, 9, 4, 9, 2, 1]
            [9, 8, 5, 6, 7, 8, 9, 8, 9, 2]
            [8, 7, 6, 7, 8, 9, 6, 7, 8, 9]
            [9, 8, 9, 9, 9, 6, 5, 6, 7, 8]
        ]
    }
}
