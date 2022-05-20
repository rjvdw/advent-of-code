use std::convert::TryFrom;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let jumps = File::open(&args[1]).read_lines(1).collect::<Vec<i32>>();

    println!(
        "[v1] It takes {} jumps to halt.",
        follow_jumps(&jumps, next_offset_v1)
    );
    println!(
        "[v2] It takes {} jumps to halt.",
        follow_jumps(&jumps, next_offset_v2)
    );
}

fn follow_jumps(jumps: &[i32], next_offset: fn(&mut i32)) -> usize {
    let mut jumps = jumps.to_vec();
    let mut idx = 0;
    let mut counter = 0;

    while let Some(offset) = safe_get_mut(&mut jumps, idx) {
        counter += 1;
        idx += *offset;
        next_offset(offset);
    }

    counter
}

fn next_offset_v1(offset: &mut i32) {
    *offset += 1;
}

fn next_offset_v2(offset: &mut i32) {
    if *offset >= 3 {
        *offset -= 1;
    } else {
        *offset += 1;
    }
}

fn safe_get_mut(jumps: &mut [i32], idx: i32) -> Option<&mut i32> {
    match usize::try_from(idx).ok() {
        Some(idx) => jumps.get_mut(idx),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_follow_jumps_v1() {
        assert_eq!(follow_jumps(&[0, 3, 0, 1, -3], next_offset_v1), 5);
    }

    #[test]
    fn test_follow_jumps_v2() {
        assert_eq!(follow_jumps(&[0, 3, 0, 1, -3], next_offset_v2), 10);
    }
}
