use crate::edges::EdgeName;

pub const FLIP_BITMASK: u8 = 0b00000100;
pub const ROTATION_BITMASK: u8 = 0b00000011;

/// Determines how a tile should be reoriented so that its `name2` edge lines up with the `name1`
/// edge of some other tile.
pub fn get_orientation(name1: EdgeName, name2: EdgeName) -> u8 {
    match name1 {
        EdgeName::Top => match name2 {
            EdgeName::Top => 0b101,
            EdgeName::Right => 0b100,
            EdgeName::Bottom => 0b000,
            EdgeName::Left => 0b001,
            EdgeName::TopRev => 0b010,
            EdgeName::RightRev => 0b011,
            EdgeName::BottomRev => 0b111,
            EdgeName::LeftRev => 0b110,
        },
        EdgeName::Right => match name2 {
            EdgeName::Top => 0b100,
            EdgeName::Right => 0b111,
            EdgeName::Bottom => 0b011,
            EdgeName::Left => 0b000,
            EdgeName::TopRev => 0b001,
            EdgeName::RightRev => 0b010,
            EdgeName::BottomRev => 0b110,
            EdgeName::LeftRev => 0b101,
        },
        EdgeName::Bottom => match name2 {
            EdgeName::Top => 0b000,
            EdgeName::Right => 0b001,
            EdgeName::Bottom => 0b101,
            EdgeName::Left => 0b100,
            EdgeName::TopRev => 0b111,
            EdgeName::RightRev => 0b110,
            EdgeName::BottomRev => 0b010,
            EdgeName::LeftRev => 0b011,
        },
        EdgeName::Left => match name2 {
            EdgeName::Top => 0b011,
            EdgeName::Right => 0b000,
            EdgeName::Bottom => 0b100,
            EdgeName::Left => 0b111,
            EdgeName::TopRev => 0b110,
            EdgeName::RightRev => 0b101,
            EdgeName::BottomRev => 0b001,
            EdgeName::LeftRev => 0b010,
        },

        EdgeName::TopRev => match name2 {
            EdgeName::Top => 0b010,
            EdgeName::Right => 0b011,
            EdgeName::Bottom => 0b111,
            EdgeName::Left => 0b110,
            EdgeName::TopRev => 0b101,
            EdgeName::RightRev => 0b100,
            EdgeName::BottomRev => 0b000,
            EdgeName::LeftRev => 0b001,
        },
        EdgeName::RightRev => match name2 {
            EdgeName::Top => 0b001,
            EdgeName::Right => 0b010,
            EdgeName::Bottom => 0b110,
            EdgeName::Left => 0b101,
            EdgeName::TopRev => 0b100,
            EdgeName::RightRev => 0b111,
            EdgeName::BottomRev => 0b011,
            EdgeName::LeftRev => 0b000,
        },
        EdgeName::BottomRev => match name2 {
            EdgeName::Top => 0b111,
            EdgeName::Right => 0b110,
            EdgeName::Bottom => 0b010,
            EdgeName::Left => 0b011,
            EdgeName::TopRev => 0b000,
            EdgeName::RightRev => 0b001,
            EdgeName::BottomRev => 0b101,
            EdgeName::LeftRev => 0b100,
        },
        EdgeName::LeftRev => match name2 {
            EdgeName::Top => 0b110,
            EdgeName::Right => 0b101,
            EdgeName::Bottom => 0b001,
            EdgeName::Left => 0b010,
            EdgeName::TopRev => 0b011,
            EdgeName::RightRev => 0b000,
            EdgeName::BottomRev => 0b100,
            EdgeName::LeftRev => 0b111,
        },
    }
}

/// Orients an x and y coordinate according to `orientation`.
pub fn orient_x_y(row: usize, column: usize, size: usize, orientation: u8) -> (usize, usize) {
    let (row, column) = match orientation & ROTATION_BITMASK {
        1 => (column, size - 1 - row),
        2 => (size - 1 - row, size - 1 - column),
        3 => (size - 1 - column, row),
        _ => (row, column),
    };

    match orientation & FLIP_BITMASK {
        0 => (row, column),
        _ => (column, row),
    }
}

// /// Reverse operation for `orient_x_y`
// pub fn invert_orient_x_y(
//     row: usize,
//     column: usize,
//     size: usize,
//     orientation: u8,
// ) -> (usize, usize) {
//     let (row, column) = match orientation & FLIP_BITMASK {
//         0 => (row, column),
//         _ => (column, row),
//     };
//
//     match orientation & ROTATION_BITMASK {
//         3 => (column, size - 1 - row),
//         2 => (size - 1 - row, size - 1 - column),
//         1 => (size - 1 - column, row),
//         _ => (row, column),
//     }
// }
