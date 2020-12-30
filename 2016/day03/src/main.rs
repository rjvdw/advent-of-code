use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::input::WithReadLines;

use crate::triangle::Triangle;

mod triangle;

fn main() {
    let args = get_args(&["<input file>"], 1);

    println!(
        "[By row] There are {} triangles that are possible.",
        parse_by_row(&args[1])
    );

    println!(
        "[By column] There are {} triangles that are possible.",
        parse_by_column(&args[1]).or_exit_with(1)
    );
}

fn parse_by_row(path: &str) -> usize {
    File::open(path)
        .read_lines::<Triangle>(1)
        .filter(|t| t.is_possible())
        .count()
}

fn parse_by_column(path: &str) -> Result<usize, ParseError> {
    let file = File::open(path)?;

    let mut triangles = Vec::new();
    let mut t1 = Vec::new();
    let mut t2 = Vec::new();
    let mut t3 = Vec::new();

    for line in BufReader::new(file).lines() {
        let line = line?;
        let mut sides = Vec::new();
        for side in line.split_whitespace() {
            sides.push(side.parse()?);
        }

        if sides.len() != 3 {
            return Err(ParseError(format!("Invalid input: {}", line)));
        }

        t1.push(sides[0]);
        t2.push(sides[1]);
        t3.push(sides[2]);

        if t1.len() == 3 {
            triangles.push(Triangle(t1[0], t1[1], t1[2]));
            t1.clear();
            triangles.push(Triangle(t2[0], t2[1], t2[2]));
            t2.clear();
            triangles.push(Triangle(t3[0], t3[1], t3[2]));
            t3.clear();
        }
    }

    Ok(triangles.iter().filter(|t| t.is_possible()).count())
}
