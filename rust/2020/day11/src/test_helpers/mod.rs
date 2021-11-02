use rdcl_aoc_helpers::input::WithAsMultilineRecords;

use crate::seat_layout::SeatLayout;

pub fn get_input(input_lines: Vec<&str>) -> SeatLayout {
    input_lines
        .as_multiline_records()
        .unwrap()
        .first()
        .cloned()
        .unwrap()
}
