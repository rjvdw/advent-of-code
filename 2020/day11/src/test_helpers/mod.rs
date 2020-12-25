use rdcl_aoc_helpers::parse::parse_multiline_input;

use crate::seat_layout::SeatLayout;

pub fn get_input(input_lines: Vec<&str>) -> SeatLayout {
    let input = parse_multiline_input::<SeatLayout>(input_lines).unwrap();
    input.first().unwrap().clone()
}
