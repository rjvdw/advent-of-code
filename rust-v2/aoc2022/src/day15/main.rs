//! The solution for [advent of code 2022, day 15](https://adventofcode.com/2022/day/15)

use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::point::Point;
use crate::sensor::Sensor;

mod line;
mod point;
mod sensor;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 15")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The row in which to count positions that cannot contain a sensor.
    #[clap(short, long, value_parser, default_value_t = 2_000_000)]
    row: i64,

    /// The lower bound for the x and y coordinates.
    #[clap(short, long, value_parser, default_value_t = 0)]
    lower_bound: i64,

    /// The upper bound for the x and y coordinates.
    #[clap(short, long, value_parser, default_value_t = 4_000_000)]
    upper_bound: i64,

    /// The factor with which to multiply the x coordinate while calculating the tuning frequency.
    #[clap(short, long, value_parser, default_value_t = 4_000_000)]
    x_factor: i64,
}

fn main() {
    let args: Args = Args::parse();
    let sensors = InputReader::from(args.input)
        .parse_lines(Sensor::from_str)
        .collect::<Vec<Sensor>>();

    let empty_spaces = count_empty_spaces(&sensors, args.row);
    println!(
        "In row {}, there are {} positions that cannot contain a sensor",
        args.row, empty_spaces
    );

    match find_distress_beacon(&sensors, args.lower_bound, args.upper_bound) {
        Some(beacon) => println!(
            "The distress beacon is at: {}, which makes its tuning frequency: {}",
            beacon,
            beacon.0 * args.x_factor + beacon.1
        ),
        None => eprintln!("No distress beacon could be found"),
    }
}

fn analyze_row(sensors: &[Sensor], row: i64) -> Vec<(i64, i64)> {
    let mut sections: Vec<(i64, i64)> = vec![];

    // find occupied sections
    for sensor in sensors {
        let distance = sensor.size();
        let d_y = (row - sensor.coordinate.1).abs();
        if d_y <= distance {
            let d_x = distance - d_y;
            let min_x = sensor.coordinate.0 - d_x;
            let max_x = sensor.coordinate.0 + d_x;
            sections.push((min_x, max_x));
        }
    }

    // merge sections
    let mut merged_sections: Vec<(i64, i64)> = vec![];
    while let Some(section1) = sections.pop() {
        let mut merged = false;
        for section2 in sections.iter_mut() {
            if section1.0 <= section2.1 && section1.1 >= section2.0 {
                section2.0 = section1.0.min(section2.0);
                section2.1 = section1.1.max(section2.1);
                merged = true;
                break;
            }
        }
        if !merged {
            merged_sections.push(section1);
        }
    }

    merged_sections
}

fn count_empty_spaces(sensors: &[Sensor], row: i64) -> usize {
    let mut beacons: HashSet<i64> = HashSet::new();

    for sensor in sensors {
        if sensor.closest_beacon.1 == row {
            beacons.insert(sensor.closest_beacon.0);
        }
    }

    analyze_row(sensors, row)
        .iter()
        .map(|&(min, max)| {
            (max + 1 - min) as usize - beacons.iter().filter(|&&i| i >= min && i <= max).count()
        })
        .sum()
}

fn find_distress_beacon(sensors: &[Sensor], lower_bound: i64, upper_bound: i64) -> Option<Point> {
    for (i, line1) in sensors.iter().flat_map(|s| s.get_edges()).enumerate() {
        for line2 in sensors.iter().flat_map(|s| s.get_edges()).skip(i + 1) {
            if let Some(point) = line1.find_intersection_point(&line2, lower_bound, upper_bound) {
                if sensors.iter().all(|sensor| !sensor.contains(&point)) {
                    return Some(point);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<Sensor> {
        InputReader::from("./src/day15/test.txt")
            .parse_lines(Sensor::from_str)
            .collect()
    }

    #[test]
    fn test_find_empty_spaces() {
        let empty_spaces = count_empty_spaces(&test_data(), 10);

        assert_eq!(empty_spaces, 26);
    }

    #[test]
    fn test_find_distress_beacon() {
        let distress_beacon = find_distress_beacon(&test_data(), 0, 20);

        assert_eq!(distress_beacon, Some(Point(14, 11)));
    }
}
