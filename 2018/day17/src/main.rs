use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadMultiLines;

use ground::Ground;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let mut ground = File::open(&args[1])
        .read_multi_lines::<Ground>(1)
        .next()
        .or_exit_with(1);

    while !ground.done() {
        ground.flow();
    }

    println!("There are {} wet tiles.", ground.get_nr_of_wet_tiles());
    println!(
        "After draining, there will be {} wet tiles.",
        ground.get_nr_of_settled_tiles()
    );
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsMultilineRecords;

    use super::*;

    #[test]
    fn test_flow_1() {
        let mut ground = get_test_input(vec![
            "x=495, y=2..7",
            "y=7, x=495..501",
            "x=501, y=3..7",
            "x=498, y=2..4",
            "x=506, y=1..2",
            "x=498, y=10..13",
            "x=504, y=10..13",
            "y=13, x=498..504",
        ]);

        while !ground.done() {
            ground.flow();
        }

        println!("{}", ground);

        assert_eq!(ground.get_nr_of_wet_tiles(), 57);
    }

    fn get_test_input(lines: Vec<&str>) -> Ground {
        lines
            .as_multiline_records::<Ground>()
            .unwrap()
            .first()
            .unwrap()
            .clone()
    }
}
