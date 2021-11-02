use std::collections::{HashMap, HashSet};
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadMultiLines;

use crate::area::Area;

mod area;

fn main() {
    let args = get_args(&["<input file>", "<max distance>"], 1);
    let area = File::open(&args[1])
        .read_multi_lines::<Area>(1)
        .next()
        .or_exit_with(1);
    let max_distance = args[2].parse::<i32>().or_exit_with(1);

    match find_largest_area(&area) {
        Some(size) => println!("The largest area has size {}.", size),
        None => eprintln!("Not able to find the largest area."),
    }

    println!(
        "The region containing all locations which have a total distance to all given coordinates of less than {} has a size of {}.",
        max_distance, find_optimal_points(&area, max_distance)
    );
}

fn find_largest_area(area: &Area) -> Option<usize> {
    let mut infinite_areas: HashSet<(i32, i32)> = HashSet::new();
    for p1 in area.edge() {
        if let Some(point) = area.get_closest(p1) {
            infinite_areas.insert(point);
        }
    }

    let mut counts: HashMap<(i32, i32), usize> = HashMap::new();
    for p1 in area.inner() {
        if let Some(point) = area.get_closest(p1) {
            *counts.entry(point).or_insert(0) += 1;
        }
    }

    counts
        .iter()
        .filter(|(point, _)| !infinite_areas.contains(*point))
        .map(|(_, count)| *count)
        .max()
}

fn find_optimal_points(area: &Area, max_distance: i32) -> usize {
    area.region(max_distance)
        .iter()
        .filter(|&p| area.distance_to_all(*p) < max_distance)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_largest_area() {
        let area = Area::new(&[(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)]);
        assert_eq!(find_largest_area(&area), Some(17));
    }

    #[test]
    fn test_find_optimal_points() {
        let area = Area::new(&[(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)]);
        assert_eq!(find_optimal_points(&area, 32), 16);
    }
}
