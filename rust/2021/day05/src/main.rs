extern crate rdcl_aoc_helpers;

use std::collections::HashMap;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::line::Line;
use crate::point::Point;

mod line;
mod point;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let lines = File::open(&args[1]).read_lines(1).collect::<Vec<Line>>();

    println!(
        "Not considering diagonals, there are {} points where multiple lines overlap.",
        count_points_where_multiple_lines_overlap(&lines, false),
    );

    println!(
        "Considering diagonals, there are {} points where multiple lines overlap.",
        count_points_where_multiple_lines_overlap(&lines, true),
    );
}

fn count_points_where_multiple_lines_overlap(lines: &[Line], include_diagonals: bool) -> usize {
    let mut counts: HashMap<Point, usize> = HashMap::new();

    lines
        .iter()
        .filter(|line| include_diagonals || line.is_horizontal() || line.is_vertical())
        .flat_map(|line| line.iter())
        .for_each(|point| *counts.entry(point).or_default() += 1);

    counts.values().filter(|&v| *v > 1).count()
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_count_points_where_multiple_lines_overlap() {
        let lines = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ]
        .as_records::<Line>()
        .unwrap();

        assert_eq!(count_points_where_multiple_lines_overlap(&lines, false), 5);
        assert_eq!(count_points_where_multiple_lines_overlap(&lines, true), 12);
    }
}
