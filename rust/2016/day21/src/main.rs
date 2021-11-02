use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::instruction::Instruction;

mod instruction;

fn main() {
    let args = get_args(&["<input file>", "<password>", "<scrambled>"], 1);
    let instructions = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Instruction>>();

    println!(
        "Your freshly scrambled password is: {}",
        scramble(&args[2], &instructions)
    );

    println!(
        "Your reverse engineered password is: {}",
        reverse(&args[3], &instructions)
    );
}

fn scramble(password: &str, instructions: &[Instruction]) -> String {
    let mut chars = password.chars().collect::<Vec<char>>();
    for instruction in instructions {
        instruction.run(&mut chars);
    }
    let mut scrambled = String::new();
    for ch in chars {
        scrambled.push(ch);
    }
    scrambled
}

fn reverse(scrambled: &str, instructions: &[Instruction]) -> String {
    let mut chars = scrambled.chars().collect::<Vec<char>>();
    for instruction in instructions.iter().rev() {
        instruction.reverse(&mut chars);
    }
    let mut password = String::new();
    for ch in chars {
        password.push(ch);
    }
    password
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_scramble() {
        let instructions = vec![
            "swap position 4 with position 0",
            "swap letter d with letter b",
            "reverse positions 0 through 4",
            "rotate left 1 step",
            "move position 1 to position 4",
            "move position 3 to position 0",
            "rotate based on position of letter b",
            "rotate based on position of letter d",
        ]
        .as_records::<Instruction>()
        .unwrap();

        assert_eq!(scramble("abcde", &instructions), "decab");
    }

    #[test]
    fn test_reverse() {
        let instructions = vec![
            "swap position 4 with position 0",
            "swap letter d with letter b",
            "reverse positions 0 through 4",
            "rotate left 1 step",
            "move position 1 to position 4",
            "move position 3 to position 0",
            "rotate based on position of letter b",
            "rotate based on position of letter d",
        ]
        .as_records::<Instruction>()
        .unwrap();

        assert_eq!(reverse("decab", &instructions), "abcde");
    }
}
