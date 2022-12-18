//! The solution for [advent of code 2022, day 18](https://adventofcode.com/2022/day/18)

extern crate core;

use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::point::Point;
use crate::region::Region;

mod area;
mod point;
mod region;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 18")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let points: Vec<Point> = InputReader::from(args.input)
        .parse_lines(Point::from_str)
        .collect();

    let regions = to_regions(&points);
    println!("The total surface area is {}", surface_area(&regions));

    let regions = flood_fill(&regions);
    println!("The external surface area is {}", surface_area(&regions));
}

fn to_regions(points: &[Point]) -> Vec<Region> {
    let mut regions: Vec<Region> = points.iter().map(|&p| Region::new(p)).collect();
    let mut simplified = vec![];

    while let Some(region) = regions.pop() {
        let mut was_joined = false;
        for other in &mut regions {
            if let Some(joined) = region.join(other) {
                *other = joined;
                was_joined = true;
                break;
            }
        }
        if !was_joined {
            simplified.push(region);
        }
    }

    simplified
}

fn flood_fill(regions: &[Region]) -> Vec<Region> {
    let (min, max) = bounds(regions);

    // keep track of all pixels that cannot be reached by the flood fill
    let mut unreachable: HashSet<Point> = min.upto(max).collect();

    // ensure there is a layer of air around all droplets
    let min = min - (1, 1, 1);
    let max = max + (1, 1, 1);

    let mut seen: HashSet<Point> = HashSet::new();
    let mut flood = vec![min];

    while let Some(point) = flood.pop() {
        if seen.contains(&point) {
            continue;
        }
        seen.insert(point);

        let neighbours = point.neighbours();
        let neighbours = neighbours
            .iter()
            .copied()
            .filter(|&p| p.within_bounds(min, max))
            .filter(|p| !seen.contains(p));

        for neighbour in neighbours {
            let is_reachable = regions.iter().all(|r| r.can_reach(point, neighbour));
            if is_reachable {
                unreachable.remove(&neighbour);
                flood.push(neighbour);
            }
        }
    }

    let points: Vec<Point> = unreachable.iter().copied().collect();
    to_regions(&points)
}

fn bounds(regions: &[Region]) -> (Point, Point) {
    let mut min = Point::new(i32::MAX, i32::MAX, i32::MAX);
    let mut max = Point::new(i32::MIN, i32::MIN, i32::MIN);

    for point in regions
        .iter()
        .flat_map(|r| r.surface.iter())
        .map(|a| a.position)
    {
        min.x = min.x.min(point.x);
        min.y = min.y.min(point.y);
        min.z = min.z.min(point.z);

        max.x = max.x.max(point.x + 1);
        max.y = max.y.max(point.y + 1);
        max.z = max.z.max(point.z + 1);
    }

    (min, max)
}

fn surface_area(regions: &[Region]) -> usize {
    regions.iter().map(|r| r.surface_area()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<Point> {
        InputReader::from("./src/day18/test.txt")
            .parse_lines(Point::from_str)
            .collect()
    }

    #[test]
    fn test_surface_area() {
        let regions = to_regions(&test_data());
        assert_eq!(surface_area(&regions), 64)
    }

    #[test]
    fn test_external_surface_area() {
        let regions = to_regions(&test_data());
        let flooded = flood_fill(&regions);
        assert_eq!(surface_area(&flooded), 58)
    }
}
