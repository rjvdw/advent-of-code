use std::collections::HashSet;
use std::fs::File;
use std::io;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::minmax::MinMax;
use crate::point::Point;

mod minmax;
mod point;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let points = File::open(&args[1]).read_lines(1).collect::<Vec<Point>>();

    let (time, message) = find_message(&points);
    println!(
        "After {} seconds, the following message appears in the sky:",
        time
    );
    print_message(&message, &mut io::stdout()).or_exit_with(1);
}

fn print_message<W: io::Write>(points: &[Point], output: &mut W) -> io::Result<()> {
    let mut message: HashSet<(i64, i64)> = HashSet::new();
    let mut x_range = MinMax::default();
    let mut y_range = MinMax::default();

    for point in points {
        let (x, y) = point.get_position();
        message.insert((x, y));
        x_range.update(x);
        y_range.update(y);
    }

    for y in y_range.range_inclusive() {
        for x in x_range.range_inclusive() {
            if message.contains(&(x, y)) {
                write!(output, "#")?;
            } else {
                write!(output, ".")?;
            }
        }
        writeln!(output)?;
    }

    Ok(())
}

fn find_message(points: &[Point]) -> (usize, Vec<Point>) {
    let mut time = 0;
    let mut points = points.to_vec();
    let mut prev_height = i64::MAX;
    let mut current_height = height(&points);

    while current_height < prev_height {
        time += 1;
        tick(&mut points);
        prev_height = current_height;
        current_height = height(&points);
    }
    rewind(&mut points);

    (time - 1, points)
}

fn height(points: &[Point]) -> i64 {
    let mut y_range = MinMax::default();
    for (_, y) in points.iter().map(|p| p.get_position()) {
        y_range.update(y);
    }
    y_range.len_inclusive()
}

fn tick(points: &mut [Point]) {
    for point in points {
        point.tick();
    }
}

fn rewind(points: &mut [Point]) {
    for point in points {
        point.rewind();
    }
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_tick_and_height() {
        let mut points = get_test_input();

        assert_eq!(height(&points), 16);
        tick(&mut points);
        assert_eq!(height(&points), 12);
        tick(&mut points);
        assert_eq!(height(&points), 10);
        tick(&mut points);
        assert_eq!(height(&points), 8);
        tick(&mut points);
        assert_eq!(height(&points), 11);
    }

    #[test]
    fn test_find_and_print_message() {
        let points = get_test_input();
        let (time, message) = find_message(&points);

        assert_eq!(time, 3);
        assert_eq!(height(&message), 8);

        let mut out = Vec::new();
        print_message(&message, &mut out).unwrap();

        let mut expected = String::new();
        expected.push_str("#...#..###\n");
        expected.push_str("#...#...#.\n");
        expected.push_str("#...#...#.\n");
        expected.push_str("#####...#.\n");
        expected.push_str("#...#...#.\n");
        expected.push_str("#...#...#.\n");
        expected.push_str("#...#...#.\n");
        expected.push_str("#...#..###\n");

        assert_eq!(out, expected.as_bytes());
    }

    fn get_test_input() -> Vec<Point> {
        vec![
            "position=< 9,  1> velocity=< 0,  2>",
            "position=< 7,  0> velocity=<-1,  0>",
            "position=< 3, -2> velocity=<-1,  1>",
            "position=< 6, 10> velocity=<-2, -1>",
            "position=< 2, -4> velocity=< 2,  2>",
            "position=<-6, 10> velocity=< 2, -2>",
            "position=< 1,  8> velocity=< 1, -1>",
            "position=< 1,  7> velocity=< 1,  0>",
            "position=<-3, 11> velocity=< 1, -2>",
            "position=< 7,  6> velocity=<-1, -1>",
            "position=<-2,  3> velocity=< 1,  0>",
            "position=<-4,  3> velocity=< 2,  0>",
            "position=<10, -3> velocity=<-1,  1>",
            "position=< 5, 11> velocity=< 1, -2>",
            "position=< 4,  7> velocity=< 0, -1>",
            "position=< 8, -2> velocity=< 0,  1>",
            "position=<15,  0> velocity=<-2,  0>",
            "position=< 1,  6> velocity=< 1,  0>",
            "position=< 8,  9> velocity=< 0, -1>",
            "position=< 3,  3> velocity=<-1,  1>",
            "position=< 0,  5> velocity=< 0, -1>",
            "position=<-2,  2> velocity=< 2,  0>",
            "position=< 5, -2> velocity=< 1,  2>",
            "position=< 1,  4> velocity=< 2,  1>",
            "position=<-2,  7> velocity=< 2, -2>",
            "position=< 3,  6> velocity=<-1, -1>",
            "position=< 5,  0> velocity=< 1,  0>",
            "position=<-6,  0> velocity=< 2,  0>",
            "position=< 5,  9> velocity=< 1, -2>",
            "position=<14,  7> velocity=<-2,  0>",
            "position=<-3,  6> velocity=< 2, -1>",
        ]
        .as_records::<Point>()
        .unwrap()
    }
}
