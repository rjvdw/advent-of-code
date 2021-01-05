use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadMultiLines;

use crate::rail_map::RailMap;

mod rail_map;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let mut rail_map = File::open(&args[1])
        .read_multi_lines::<RailMap>(1)
        .next()
        .or_exit_with(1);

    let crashed = skip_to_first_crash(&mut rail_map);
    println!("The first crash happens at [{},{}].", crashed.0, crashed.1);

    match skip_until_only_one_cart_remaining(&mut rail_map) {
        Some(final_cart) => println!(
            "The final cart remaining ends up at [{},{}].",
            final_cart.0, final_cart.1
        ),
        None => println!("All carts end up crashing."),
    }
}

/// Fast forward to the point where the first crash happens, and return its coordinates.
fn skip_to_first_crash(rail_map: &mut RailMap) -> (usize, usize) {
    while rail_map.is_crash_free() {
        rail_map.tick();
    }
    rail_map.get_first_crash().unwrap()
}

/// Fast forward until there is only one cart left, and return its coordinates. Will return None if
/// all carts end up crashing.
fn skip_until_only_one_cart_remaining(rail_map: &mut RailMap) -> Option<(usize, usize)> {
    while rail_map.get_nr_carts_remaining() > 1 {
        rail_map.tick();
    }
    rail_map.get_last_remaining_cart()
}

#[cfg(test)]
#[rustfmt::skip::macros(vec)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsMultilineRecords;

    use super::*;

    #[test]
    fn test_skip_to_first_crash() {
        let mut rail_map = get_test_input(vec![
            r"/->-\        ",
            r"|   |  /----\",
            r"| /-+--+-\  |",
            r"| | |  | v  |",
            r"\-+-/  \-+--/",
            r"  \------/   ",
        ]);
        assert_eq!(skip_to_first_crash(&mut rail_map), (7, 3));
    }

    #[test]
    fn test_skip_until_only_one_cart_remaining() {
        let mut rail_map = get_test_input(vec![
            r"/>-<\  ",
            r"|   |  ",
            r"| /<+-\",
            r"| | | v",
            r"\>+</ |",
            r"  |   ^",
            r"  \<->/",
        ]);
        assert_eq!(
            skip_until_only_one_cart_remaining(&mut rail_map),
            Some((6, 4))
        )
    }

    fn get_test_input(lines: Vec<&str>) -> RailMap {
        lines
            .as_multiline_records::<RailMap>()
            .unwrap()
            .first()
            .unwrap()
            .clone()
    }
}
