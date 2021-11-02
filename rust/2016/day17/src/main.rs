use std::collections::HashSet;

use rdcl_aoc_helpers::args::get_args;

const OPEN_DOORS: [char; 5] = ['b', 'c', 'd', 'e', 'f'];
const START: (u8, u8) = (1, 1);
const END: (u8, u8) = (4, 4);
const WIDTH: u8 = 4;
const HEIGHT: u8 = 4;

fn main() {
    let args = get_args(&["<passcode>"], 1);
    let passcode = &args[1];

    match find_shortest_path(passcode) {
        Some(path) => println!("The shortest path is {}.", path),
        None => eprintln!("There is no path for your passcode"),
    }

    match find_longest_path(passcode) {
        Some(path) => println!("The longest path has length {}.", path.len()),
        None => eprintln!("There is no path for your passcode"),
    }
}

fn find_shortest_path(passcode: &str) -> Option<String> {
    let mut to_explore: HashSet<(String, (u8, u8))> = HashSet::new();
    to_explore.insert((String::new(), START));

    while !to_explore.is_empty() {
        let mut next_to_explore = HashSet::new();
        for item in to_explore {
            let (path, xy) = item;
            for door in get_doors(passcode, &path, xy) {
                if door.1 == END {
                    return Some(door.0);
                }
                next_to_explore.insert(door);
            }
        }
        to_explore = next_to_explore;
    }

    None
}

fn find_longest_path(passcode: &str) -> Option<String> {
    let mut to_explore: HashSet<(String, (u8, u8))> = HashSet::new();
    to_explore.insert((String::new(), START));

    let mut longest_path = None;

    while !to_explore.is_empty() {
        let mut next_to_explore = HashSet::new();
        for item in to_explore {
            let (path, xy) = item;
            for door in get_doors(passcode, &path, xy) {
                if door.1 == END {
                    longest_path = Some(door.0);
                } else {
                    next_to_explore.insert(door);
                }
            }
        }
        to_explore = next_to_explore;
    }

    longest_path
}

fn get_doors(passcode: &str, path: &str, (x, y): (u8, u8)) -> Vec<(String, (u8, u8))> {
    let input = format!("{}{}", passcode, path);
    let hash = format!("{:x}", md5::compute(input));
    let mut doors = Vec::new();
    for (idx, ch) in hash.chars().take(4).enumerate() {
        if OPEN_DOORS.contains(&ch) {
            let (dir, next_xy) = match idx {
                0 => ('U', (x, y - 1)),
                1 => ('D', (x, y + 1)),
                2 => ('L', (x - 1, y)),
                3 => ('R', (x + 1, y)),
                _ => unreachable!(),
            };

            if next_xy.0 >= 1 && next_xy.0 <= WIDTH && next_xy.1 >= 1 && next_xy.1 <= HEIGHT {
                let mut next_path = path.to_string();
                next_path.push(dir);
                doors.push((next_path, next_xy));
            }
        }
    }
    doors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_shortest_path() {
        assert_eq!(find_shortest_path("hijkl"), None);
        assert_eq!(find_shortest_path("ihgpwlah").unwrap(), "DDRRRD");
        assert_eq!(find_shortest_path("kglvqrro").unwrap(), "DDUDRLRRUDRD");
        assert_eq!(
            find_shortest_path("ulqzkmiv").unwrap(),
            "DRURDRUDDLLDLUURRDULRLDUUDDDRR"
        );
    }

    #[test]
    fn test_find_longest_path() {
        assert_eq!(find_longest_path("hijkl"), None);
        assert_eq!(find_longest_path("ihgpwlah").unwrap().len(), 370);
        assert_eq!(find_longest_path("kglvqrro").unwrap().len(), 492);
        assert_eq!(find_longest_path("ulqzkmiv").unwrap().len(), 830);
    }
}
