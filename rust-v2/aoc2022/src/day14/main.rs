//! The solution for [advent of code 2022, day 14](https://adventofcode.com/2022/day/14)

use std::collections::HashSet;
use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::MainResult;

use crate::point::Point;

mod point;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 14")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The origin of the sand.
    #[clap(long, value_parser, default_value = "500,0")]
    origin: Point,

    /// Draw the cave?
    #[clap(short, long, value_parser, default_value_t = false)]
    print: bool,

    /// How many steps below the lowest point is the bottom?
    #[clap(short = 'o', long = "offset", value_parser, default_value_t = 2)]
    bottom_offset: usize,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);
    let (rocks, bottom) = parse(input.read_lines(), args.origin, args.bottom_offset)?;

    let sand = fill_cave(&rocks, args.origin, bottom, false);
    if args.print {
        draw_cave(&rocks, &sand);
    }
    println!(
        "A total of {} units of sand have come to rest before it starts flowing into the abyss",
        sand.len()
    );

    let sand = fill_cave(&rocks, args.origin, bottom, true);
    if args.print {
        draw_cave(&rocks, &sand);
    }
    println!(
        "A total of {} units of sand have come to rest before it blocks the origin",
        sand.len()
    );

    Ok(())
}

fn fill_cave(
    rocks: &HashSet<Point>,
    origin: Point,
    lowest_point: usize,
    allow_overflow: bool,
) -> HashSet<Point> {
    let mut sand: HashSet<Point> = HashSet::new();
    let mut sand_path = vec![origin];

    while let Some(point) = sand_path.pop() {
        let mut next_point = point.down();
        if !allow_overflow && next_point.1 > lowest_point {
            // all done
            break;
        }

        if rocks.contains(&next_point) || sand.contains(&next_point) {
            next_point = point.down_left();
        }
        if rocks.contains(&next_point) || sand.contains(&next_point) {
            next_point = point.down_right();
        }
        if rocks.contains(&next_point) || sand.contains(&next_point) {
            sand.insert(point);
            continue;
        }

        sand_path.push(point);
        sand_path.push(next_point);
    }

    sand
}

fn parse<T>(
    input: T,
    origin: Point,
    bottom_offset: usize,
) -> Result<(HashSet<Point>, usize), ParseError>
where
    T: Iterator<Item = String>,
{
    let mut rocks = HashSet::new();
    let mut lowest_point = 0;

    for line in input {
        let mut points = line.split(" -> ");
        let mut prev = points.next().unwrap().parse::<Point>()?;
        rocks.insert(prev);
        if prev.1 > lowest_point {
            lowest_point = prev.1;
        }

        for point in points {
            let point = point.parse::<Point>()?;
            if point.1 > lowest_point {
                lowest_point = point.1;
            }
            while prev != point {
                prev = prev.next_point(&point);
                rocks.insert(prev);
            }
        }
    }

    let bottom = lowest_point + bottom_offset;
    let min_x = origin.0 - bottom;
    let max_x = origin.0 + bottom;
    for x in min_x..=max_x {
        rocks.insert(Point(x, bottom));
    }

    Ok((rocks, lowest_point))
}

fn draw_cave(rocks: &HashSet<Point>, sand: &HashSet<Point>) {
    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;

    for point in rocks {
        if point.0 < min_x {
            min_x = point.0;
        }
        if point.0 > max_x {
            max_x = point.0;
        }
        if point.1 > max_y {
            max_y = point.1;
        }
    }

    for y in 0..=max_y {
        for x in min_x..=max_x {
            let point = Point(x, y);
            if rocks.contains(&point) {
                print!("#");
            } else if sand.contains(&point) {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = String> {
        InputReader::from("./src/day14/test.txt").read_lines()
    }

    fn test_rocks() -> HashSet<Point> {
        HashSet::from([
            // section 1
            Point(498, 4),
            Point(498, 5),
            Point(498, 6),
            Point(497, 6),
            Point(496, 6),
            // section 2
            Point(503, 4),
            Point(502, 4),
            Point(502, 5),
            Point(502, 6),
            Point(502, 7),
            Point(502, 8),
            Point(502, 9),
            Point(501, 9),
            Point(500, 9),
            Point(499, 9),
            Point(498, 9),
            Point(497, 9),
            Point(496, 9),
            Point(495, 9),
            Point(494, 9),
            // floor
            Point(489, 11),
            Point(490, 11),
            Point(491, 11),
            Point(492, 11),
            Point(493, 11),
            Point(494, 11),
            Point(495, 11),
            Point(496, 11),
            Point(497, 11),
            Point(498, 11),
            Point(499, 11),
            Point(500, 11),
            Point(501, 11),
            Point(502, 11),
            Point(503, 11),
            Point(504, 11),
            Point(505, 11),
            Point(506, 11),
            Point(507, 11),
            Point(508, 11),
            Point(509, 11),
            Point(510, 11),
            Point(511, 11),
        ])
    }

    #[test]
    fn test_parse() {
        let actual = parse(test_data(), Point(500, 0), 2).unwrap();
        let expected = (test_rocks(), 9);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_fill_without_overflow() {
        let sand = fill_cave(&test_rocks(), Point(500, 0), 9, false);
        assert_eq!(sand.len(), 24);
    }

    #[test]
    fn test_fill_with_overflow() {
        let sand = fill_cave(&test_rocks(), Point(500, 0), 9, true);
        assert_eq!(sand.len(), 93);
    }
}
