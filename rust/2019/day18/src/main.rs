use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use crate::cave::four_way::FourWayCave;
use crate::cave::simple::SimpleCave;
use crate::cave::Cave;

mod cave;

fn main() {
    let args = get_args(&["<input file part 1>", "<input file part 2>"], 1);

    let file_p1 = File::open(&args[1]).or_exit_with(1);
    let cave_p1 = SimpleCave::parse(file_p1).or_exit_with(1);

    let file_p2 = File::open(&args[2]).or_exit_with(1);
    let cave_p2 = FourWayCave::parse(file_p2).or_exit_with(1);

    // println!("part 1:");
    // println!("{}", cave_p1);

    // println!("part 2:");
    // println!("{}", cave_p2);

    match cave_p1.find_shortest_path() {
        Some(distance) => println!(
            "[part 1] The shortest path that finds all keys has length {}.",
            distance
        ),
        None => eprintln!("There is no path that finds all keys."),
    }
    match cave_p2.find_shortest_path() {
        Some(distance) => println!(
            "[part 2] The shortest path that finds all keys has length {}.",
            distance
        ),
        None => eprintln!("There is no path that finds all keys."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input_string = vec!["#########", "#b.A.@.a#", "#########"];
        let cave = SimpleCave::parse(input_string.join("\n").as_bytes()).unwrap();
        assert_eq!(cave.find_shortest_path(), Some(8));
    }

    #[test]
    fn test_2() {
        let input_string = vec![
            "########################",
            "#f.D.E.e.C.b.A.@.a.B.c.#",
            "######################.#",
            "#d.....................#",
            "########################",
        ];
        let cave = SimpleCave::parse(input_string.join("\n").as_bytes()).unwrap();
        assert_eq!(cave.find_shortest_path(), Some(86));
    }

    #[test]
    fn test_3() {
        let input_string = vec![
            "########################",
            "#...............b.C.D.f#",
            "#.######################",
            "#.....@.a.B.c.d.A.e.F.g#",
            "########################",
        ];
        let cave = SimpleCave::parse(input_string.join("\n").as_bytes()).unwrap();
        assert_eq!(cave.find_shortest_path(), Some(132));
    }

    #[test]
    fn test_4() {
        let input_string = vec![
            "#################",
            "#i.G..c...e..H.p#",
            "########.########",
            "#j.A..b...f..D.o#",
            "########@########",
            "#k.E..a...g..B.n#",
            "########.########",
            "#l.F..d...h..C.m#",
            "#################",
        ];
        let cave = SimpleCave::parse(input_string.join("\n").as_bytes()).unwrap();
        assert_eq!(cave.find_shortest_path(), Some(136));
    }

    #[test]
    fn test_5() {
        let input_string = vec![
            "########################",
            "#@..............ac.GI.b#",
            "###d#e#f################",
            "###A#B#C################",
            "###g#h#i################",
            "########################",
        ];
        let cave = SimpleCave::parse(input_string.join("\n").as_bytes()).unwrap();
        assert_eq!(cave.find_shortest_path(), Some(81));
    }

    #[test]
    fn test_6() {
        let input_string = vec![
            "#######", "#a.#Cd#", "##@#@##", "#######", "##@#@##", "#cB#Ab#", "#######",
        ];
        let cave = FourWayCave::parse(input_string.join("\n").as_bytes()).unwrap();
        assert_eq!(cave.find_shortest_path(), Some(8));
    }

    #[test]
    fn test_7() {
        let input_string = vec![
            "###############",
            "#d.ABC.#.....a#",
            "######@#@######",
            "###############",
            "######@#@######",
            "#b.....#.....c#",
            "###############",
        ];
        let cave = FourWayCave::parse(input_string.join("\n").as_bytes()).unwrap();
        assert_eq!(cave.find_shortest_path(), Some(24));
    }

    #[test]
    fn test_8() {
        let input_string = vec![
            "#############",
            "#DcBa.#.GhKl#",
            "#.###@#@#I###",
            "#e#d#####j#k#",
            "###C#@#@###J#",
            "#fEbA.#.FgHi#",
            "#############",
        ];
        let cave = FourWayCave::parse(input_string.join("\n").as_bytes()).unwrap();
        assert_eq!(cave.find_shortest_path(), Some(32));
    }

    #[test]
    fn test_9() {
        let input_string = vec![
            "#############",
            "#g#f.D#..h#l#",
            "#F###e#E###.#",
            "#dCba@#@BcIJ#",
            "#############",
            "#nK.L@#@G...#",
            "#M###N#H###.#",
            "#o#m..#i#jk.#",
            "#############",
        ];
        let cave = FourWayCave::parse(input_string.join("\n").as_bytes()).unwrap();
        assert_eq!(cave.find_shortest_path(), Some(72));
    }
}
