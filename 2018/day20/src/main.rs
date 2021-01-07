use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::room_layout::RoomLayout;

mod room_layout;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let layout = File::open(&args[1])
        .read_lines::<RoomLayout>(1)
        .next()
        .or_exit_with(1);

    println!(
        "Furthest room requires passing {} doors.",
        layout.find_longest_path()
    );

    println!(
        "There are {} rooms that require you to pass through at least 1000 doors.",
        layout.find_paths_longer_than(999)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_doors_example_1() {
        let layout = "^WNE$".parse::<RoomLayout>().unwrap();
        println!("{}", layout);
        assert_eq!(layout.find_longest_path(), 3);
    }

    #[test]
    fn test_count_doors_example_2() {
        let layout = "^ENWWW(NEEE|SSE(EE|N))$".parse::<RoomLayout>().unwrap();
        println!("{}", layout);
        assert_eq!(layout.find_longest_path(), 10);
    }

    #[test]
    fn test_count_doors_example_3() {
        let layout = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"
            .parse::<RoomLayout>()
            .unwrap();
        println!("{}", layout);
        assert_eq!(layout.find_longest_path(), 18);
    }

    #[test]
    fn test_count_doors_example_4() {
        let layout = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"
            .parse::<RoomLayout>()
            .unwrap();
        println!("{}", layout);
        assert_eq!(layout.find_longest_path(), 23);
    }

    #[test]
    fn test_count_doors_example_5() {
        let layout = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"
            .parse::<RoomLayout>()
            .unwrap();
        println!("{}", layout);
        assert_eq!(layout.find_longest_path(), 31);
    }
}
