use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::{MappedLines, WithReadLines};

use crate::present::Present;

mod present;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let presents = File::open(&args[1]).read_lines::<Present>(1);

    let (wrapping_paper, ribbon) = calculate_required_wrapping_paper(presents);
    println!("To wrap all the presents, we need {} square feet of wrapping paper, and {} feet of ribbon.", wrapping_paper, ribbon);
}

fn calculate_required_wrapping_paper(presents: MappedLines<Present, File>) -> (u64, u64) {
    presents
        .map(|present| present.get_required_materials())
        .fold((0, 0), |acc, (paper, ribbon)| {
            (acc.0 + paper as u64, acc.1 + ribbon as u64)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_wrapping_paper() {
        let present = "2x3x4".parse::<Present>().unwrap();
        assert_eq!(present.get_required_wrapping_paper(), 58);

        let present = "1x1x10".parse::<Present>().unwrap();
        assert_eq!(present.get_required_wrapping_paper(), 43);
    }

    #[test]
    fn test_required_ribbon() {
        let present = "2x3x4".parse::<Present>().unwrap();
        assert_eq!(present.get_required_ribbon(), 34);

        let present = "1x1x10".parse::<Present>().unwrap();
        assert_eq!(present.get_required_ribbon(), 14);
    }
}
