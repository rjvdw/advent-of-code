use std::collections::{HashSet, VecDeque};

use rdcl_aoc_helpers::args::get_args;

use shared::knot_hash::compute_knot_hash;

const GRID_SIZE: usize = 128;

fn main() {
    let args = get_args(&["<key string>"], 1);
    let grid = compute_grid(&args[1]);

    println!(
        "The number of squares that are used is {}.",
        grid.iter().map(|h| h.count_ones()).sum::<u32>()
    );
    println!("There are {} regions in this grid.", count_regions(&grid))
}

fn compute_grid(key_string: &str) -> [u128; GRID_SIZE] {
    let mut grid = [0; GRID_SIZE];
    for (i, hash) in grid.iter_mut().enumerate() {
        let row = format!("{}-{}", key_string, i);
        *hash = compute_knot_hash(row.as_bytes());
    }
    grid
}

fn count_regions(grid: &[u128; GRID_SIZE]) -> usize {
    let mut used_squares: HashSet<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let mut row = *row;
            let mut squares: Vec<(usize, usize)> = vec![];
            for x in 0..GRID_SIZE {
                let x = GRID_SIZE - x - 1;
                if row % 2 == 1 {
                    squares.push((x, y));
                }
                row >>= 1;
            }
            squares
        })
        .collect();

    let mut regions = 0;
    let mut exploring = VecDeque::new();

    while let Some(square) = used_squares.iter().next() {
        regions += 1;
        exploring.clear();
        exploring.push_back(*square);

        while let Some((x, y)) = exploring.pop_front() {
            used_squares.remove(&(x, y));
            for neighbour in get_neighbours((x, y)) {
                if used_squares.contains(&neighbour) {
                    exploring.push_back(neighbour);
                }
            }
        }
    }

    regions
}

fn get_neighbours((x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    if x > 0 {
        neighbours.push((x - 1, y));
    }
    if x + 1 < GRID_SIZE {
        neighbours.push((x + 1, y));
    }
    if y > 0 {
        neighbours.push((x, y - 1));
    }
    if y + 1 < GRID_SIZE {
        neighbours.push((x, y + 1));
    }
    neighbours
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_grid() {
        let grid = compute_grid("flqrgnkx");
        assert_eq!(grid.iter().map(|h| h.count_ones()).sum::<u32>(), 8108);
    }

    #[test]
    fn test_count_regions() {
        let grid = compute_grid("flqrgnkx");
        assert_eq!(count_regions(&grid), 1242);
    }
}
