use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::room::Room;

mod room;

fn main() {
    let args = get_args(&["<Input file>"], 1);
    let rooms = File::open(&args[1]).read_lines::<Room>(1);

    let valid_rooms: Vec<Room> = rooms.filter(Room::is_valid).collect();

    println!(
        "The sum of the sectors of all the valid rooms is {}.",
        valid_rooms.iter().map(Room::get_sector).sum::<u32>()
    );

    for room in valid_rooms {
        let decrypted = room.decrypt();

        if decrypted.contains("north") && decrypted.contains("pole") {
            println!(
                "Room \"{}\" could be it (sector = {})",
                decrypted,
                room.get_sector()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_1() {
        let room = "aaaaa-bbb-z-y-x-123[abxyz]".parse::<Room>().unwrap();
        assert!(room.is_valid());
    }

    #[test]
    fn test_is_valid_2() {
        let room = "a-b-c-d-e-f-g-h-987[abcde]".parse::<Room>().unwrap();
        assert!(room.is_valid());
    }

    #[test]
    fn test_is_valid_3() {
        let room = "not-a-real-room-404[oarel]".parse::<Room>().unwrap();
        assert!(room.is_valid());
    }

    #[test]
    fn test_is_valid_4() {
        let room = "totally-real-room-200[decoy]".parse::<Room>().unwrap();
        assert!(!room.is_valid());
    }

    #[test]
    fn test_decrypt() {
        let room = "qzmt-zixmtkozy-ivhz-343[abcde]".parse::<Room>().unwrap();
        assert_eq!(room.decrypt(), "very encrypted name".to_string());
    }
}
