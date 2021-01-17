use std::collections::HashSet;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use game_of_life::grid::Grid;
use game_of_life::recursive_grid::RecursiveGrid;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let file = File::open(&args[1]).or_exit_with(1);
    let grid = Grid::parse(file).or_exit_with(1);
    println!(
        "The first biodiversity rating that appears twice is {}.",
        find_stable_situation(grid)
    );

    let file = File::open(&args[1]).or_exit_with(1);
    let grid = RecursiveGrid::parse(file).or_exit_with(1);
    println!(
        "After 200 minutes, there are {} bugs.",
        count_bugs_after(grid, 200)
    )
}

fn find_stable_situation(mut grid: Grid) -> u64 {
    let mut seen = HashSet::new();
    seen.insert(grid.clone());
    loop {
        grid = grid.tick();
        if seen.contains(&grid) {
            return grid.calculate_biodiversity_rating();
        }
        seen.insert(grid.clone());
    }
}

fn count_bugs_after(mut grid: RecursiveGrid, count: usize) -> usize {
    for _ in 0..count {
        grid = grid.tick();
    }
    grid.count_bugs()
}

#[cfg(test)]
#[rustfmt::skip::macros(vec)]
mod tests {
    use super::*;

    #[test]
    fn test_find_stable_situation() {
        let test_input = vec![
            "....#",
            "#..#.",
            "#..##",
            "..#..",
            "#....",
        ];
        let grid = Grid::parse(test_input.join("\n").as_bytes()).unwrap();
        assert_eq!(find_stable_situation(grid), 2129920);
    }
}
