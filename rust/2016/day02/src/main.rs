use std::collections::HashMap;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::key::Key;

mod key;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let keys = File::open(&args[1]).read_lines(1).collect::<Vec<Key>>();

    println!(
        "[Rectangular keypad] The code to the bathroom is {}.",
        solve_for_rectangular_keypad(&keys)
    );
    println!(
        "[Actual keypad] The code to the bathroom is {}.",
        solve_for_actual_keypad(&keys)
    );
}

fn solve_for_rectangular_keypad(keys: &[Key]) -> String {
    let mut keypad = HashMap::new();
    keypad.insert((1, 1), "1".to_string());
    keypad.insert((1, 2), "2".to_string());
    keypad.insert((1, 3), "3".to_string());
    keypad.insert((2, 1), "4".to_string());
    keypad.insert((2, 2), "5".to_string());
    keypad.insert((2, 3), "6".to_string());
    keypad.insert((3, 1), "7".to_string());
    keypad.insert((3, 2), "8".to_string());
    keypad.insert((3, 3), "9".to_string());

    let mut pos = (2, 2);
    let mut code = String::new();
    for key in keys {
        let result = key.solve_for_keypad(&keypad, pos);
        code.push_str(&result.0);
        pos = result.1;
    }
    code
}

fn solve_for_actual_keypad(keys: &[Key]) -> String {
    let mut keypad = HashMap::new();
    keypad.insert((1, 3), "1".to_string());
    keypad.insert((2, 2), "2".to_string());
    keypad.insert((2, 3), "3".to_string());
    keypad.insert((2, 4), "4".to_string());
    keypad.insert((3, 1), "5".to_string());
    keypad.insert((3, 2), "6".to_string());
    keypad.insert((3, 3), "7".to_string());
    keypad.insert((3, 4), "8".to_string());
    keypad.insert((3, 5), "9".to_string());
    keypad.insert((4, 2), "A".to_string());
    keypad.insert((4, 3), "B".to_string());
    keypad.insert((4, 4), "C".to_string());
    keypad.insert((5, 3), "D".to_string());

    let mut pos = (3, 1);
    let mut code = String::new();
    for key in keys {
        let result = key.solve_for_keypad(&keypad, pos);
        code.push_str(&result.0);
        pos = result.1;
    }
    code
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_rectangular_keypad() {
        let keys = vec!["ULL", "RRDDD", "LURDL", "UUUUD"]
            .as_records::<Key>()
            .unwrap();

        let code = solve_for_rectangular_keypad(&keys);

        assert_eq!(code, "1985".to_string());
    }

    #[test]
    fn test_actual_keypad() {
        let keys = vec!["ULL", "RRDDD", "LURDL", "UUUUD"]
            .as_records::<Key>()
            .unwrap();

        let code = solve_for_actual_keypad(&keys);

        assert_eq!(code, "5DB3".to_string());
    }
}
