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

    println!("Part 1:");
    for (i, v1) in values.iter().enumerate() {
        for v2 in values.iter().skip(i + 1) {
            if v1 + v2 == 2020 {
                println!("  {} + {} = 2020, {} * {} = {}", v1, v2, v1, v2, v1 * v2);
            }
        }
    }

    println!("Part 2:");
    for (i, v1) in values.iter().enumerate() {
        for (j, v2) in values.iter().skip(i + 1).enumerate() {
            for v3 in values.iter().skip(j + 1) {
                if v1 + v2 + v3 == 2020 {
                    println!("  {} + {} + {} = 2020, {} * {} * {} = {}", v1, v2, v3, v1, v2, v3, v1 * v2 * v3);
                }
            }
        }
    }
}
