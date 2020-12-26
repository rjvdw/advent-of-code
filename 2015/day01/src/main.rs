use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let input = File::open(&args[1]).read_lines::<String>(1).next().unwrap();

    println!("Santa ends up on floor {}", floor_count(&input));
    match find_first_basement_position(&input) {
        Some(pos) => println!(
            "The first position that leads Santa into the basement is {}.",
            pos
        ),
        None => println!("Santa never reaches the basement."),
    }
}

fn floor_count(input: &str) -> i32 {
    let mut floor = 0;
    for ch in input.chars() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {} // ignore
        }
    }
    floor
}

fn find_first_basement_position(input: &str) -> Option<usize> {
    let mut floor = 0;
    for (pos, ch) in input.chars().enumerate() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {} // ignore
        }
        if floor < 0 {
            return Some(pos + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floor_count() {
        assert_eq!(floor_count("(())"), 0);
        assert_eq!(floor_count("()()"), 0);
        assert_eq!(floor_count("((("), 3);
        assert_eq!(floor_count("(()(()("), 3);
        assert_eq!(floor_count("))((((("), 3);
        assert_eq!(floor_count("())"), -1);
        assert_eq!(floor_count("))("), -1);
        assert_eq!(floor_count(")))"), -3);
        assert_eq!(floor_count(")())())"), -3);
    }

    #[test]
    fn test_find_first_basement_position() {
        assert_eq!(find_first_basement_position(")"), Some(1));
        assert_eq!(find_first_basement_position("()())"), Some(5));
    }
}
