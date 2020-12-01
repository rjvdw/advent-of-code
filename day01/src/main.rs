use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").expect("Cannot read input");
    let mut values = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.expect("Unable to read line");
        let line = line.parse::<i32>().expect("Unable to parse value");
        values.push(line);
    }

    match solve(&values, 0, 2, 2020) {
        Some(v) => println!("Part 1: {}", v),
        None => println!("Part 1: No solution")
    }

    match solve(&values, 0, 3, 2020) {
        Some(v) => println!("Part 2: {}", v),
        None => println!("Part 2: No solution")
    }
}

fn solve(values: &Vec<i32>, from: usize, level: usize, sum: i32) -> Option<i32> {
    if level <= 1 {
        for &v in values.iter().skip(from + 1) {
            if v == sum {
                return Some(v);
            }
        }
    } else {
        for (pos, &v) in values.iter().skip(from + 1).enumerate() {
            match solve(values, pos, level - 1, sum - v) {
                Some(r) => return Some(r * v),
                None => {}
            }
        }
    }

    return None;
}
