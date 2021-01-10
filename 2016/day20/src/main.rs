use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::parse_error;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let ranges = parse_input(&args[1]).or_exit_with(1);
    let collapsed = collapse_ranges(&ranges);
    let allowed = get_allowed(&collapsed);

    match allowed.first() {
        Some(ip) => println!(
            "{} is the first IP that is not blocked. There are {} IPs that are not blocked.",
            ip,
            allowed.len()
        ),
        None => eprintln!("All IPs are blocked."),
    }
}

fn get_allowed(ranges: &[(u32, u32)]) -> Vec<u32> {
    let mut allowed = Vec::new();
    let mut marker = 0;
    for &(low, high) in ranges {
        for ip in marker..low {
            allowed.push(ip);
        }
        if high == u32::MAX {
            return allowed;
        }
        marker = high + 1;
    }
    for ip in marker..u32::MAX {
        allowed.push(ip);
    }
    allowed
}

fn collapse_ranges(ranges: &[(u32, u32)]) -> Vec<(u32, u32)> {
    let mut collapsed = Vec::new();
    let mut iter = ranges.iter().cloned();
    if let Some(mut range) = iter.next() {
        for (from, to) in iter {
            if from >= range.0 && (range.1 == u32::MAX || from <= range.1 + 1) {
                range.1 = range.1.max(to);
            } else {
                collapsed.push(range);
                range = (from, to);
            }
        }
        collapsed.push(range);
    }
    collapsed
}

fn parse_input(path: &str) -> Result<Vec<(u32, u32)>, ParseError> {
    let file = File::open(path)?;
    let mut ranges: Vec<(u32, u32)> = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line?;
        match line.find('-') {
            Some(idx) => {
                let from = line[..idx].parse()?;
                let to = line[idx + 1..].parse()?;
                ranges.push((from, to));
            }
            None => return Err(parse_error!("Invalid input: {}", line)),
        }
    }
    ranges.sort_unstable();
    Ok(ranges)
}
