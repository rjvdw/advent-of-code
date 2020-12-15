extern crate helpers;

use std::env;
use std::process::exit;

/// https://adventofcode.com/2020/day/#
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", &args[0]);
        exit(1);
    }

    println!("Input file: {}", &args[1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn test() {
            //
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn test() {
            //
        }
    }
}
