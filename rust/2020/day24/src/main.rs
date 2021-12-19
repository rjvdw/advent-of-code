extern crate rdcl_aoc_helpers;

use std::collections::{HashMap, HashSet};
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::tile::Tile;
use crate::toggleable::Toggleable;

mod tile;
mod toggleable;

fn main() {
    let args = get_args(&["<input file>", "<nr days>"], 1);

    let input_tiles = File::open(&args[1]).read_lines(1).collect::<Vec<Tile>>();
    let nr_days = args[2].parse::<u32>().or_exit_with(1);

    let mut flipped_tiles = walk(&input_tiles);

    println!(
        "After walking the path, there are {} tiles that have been flipped.",
        flipped_tiles.len()
    );

    for _ in 0..nr_days {
        flipped_tiles = next_day(&flipped_tiles);
    }

    println!(
        "After {} days, there are {} tiles that have been flipped.",
        nr_days,
        flipped_tiles.len()
    );
}

fn walk(input_tiles: &[Tile]) -> HashSet<Tile> {
    let mut flipped_tiles = HashSet::new();
    for tile in input_tiles {
        flipped_tiles.toggle(tile);
    }
    flipped_tiles
}

fn next_day(flipped_tiles: &HashSet<Tile>) -> HashSet<Tile> {
    let mut nr_flipped_neighbours: HashMap<Tile, usize> = HashMap::new();
    for tile in flipped_tiles {
        for neighbour in &tile.neighbours() {
            *nr_flipped_neighbours.entry(*neighbour).or_insert(0) += 1;
        }
    }
    nr_flipped_neighbours
        .iter()
        .filter(|(tile, &count)| count == 2 || (count == 1 && flipped_tiles.contains(tile)))
        .map(|(&tile, _)| tile)
        .collect()
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_walk() {
        let input = get_input();
        let expected = get_expected(&[
            (-3, -3),
            (-3, -2),
            (-2, -1),
            (-2, 0),
            (-1, 1),
            (0, -2),
            (0, 0),
            (0, 1),
            (2, 0),
            (3, 3),
        ]);

        let flipped = walk(&input);

        assert_eq!(flipped, expected);
    }

    #[test]
    fn test_next_day() {
        let input = get_input();
        let mut flipped = walk(&input);

        // after day 1
        flipped = next_day(&flipped);
        assert_eq!(flipped.len(), 15);

        // after day 2
        flipped = next_day(&flipped);
        assert_eq!(flipped.len(), 12);

        // after day 3
        flipped = next_day(&flipped);
        assert_eq!(flipped.len(), 25);

        // after day 4
        flipped = next_day(&flipped);
        assert_eq!(flipped.len(), 14);

        // after day 5
        flipped = next_day(&flipped);
        assert_eq!(flipped.len(), 23);

        // after day 6
        flipped = next_day(&flipped);
        assert_eq!(flipped.len(), 28);

        // after day 7
        flipped = next_day(&flipped);
        assert_eq!(flipped.len(), 41);

        // after day 8
        flipped = next_day(&flipped);
        assert_eq!(flipped.len(), 37);

        // after day 9
        flipped = next_day(&flipped);
        assert_eq!(flipped.len(), 49);

        // after day 10
        flipped = next_day(&flipped);
        assert_eq!(flipped.len(), 37);

        // after day 20
        for _ in 0..10 {
            flipped = next_day(&flipped);
        }
        assert_eq!(flipped.len(), 132);

        // after day 30
        for _ in 0..10 {
            flipped = next_day(&flipped);
        }
        assert_eq!(flipped.len(), 259);

        // after day 40
        for _ in 0..10 {
            flipped = next_day(&flipped);
        }
        assert_eq!(flipped.len(), 406);

        // after day 50
        for _ in 0..10 {
            flipped = next_day(&flipped);
        }
        assert_eq!(flipped.len(), 566);

        // after day 60
        for _ in 0..10 {
            flipped = next_day(&flipped);
        }
        assert_eq!(flipped.len(), 788);

        // after day 70
        for _ in 0..10 {
            flipped = next_day(&flipped);
        }
        assert_eq!(flipped.len(), 1106);

        // after day 80
        for _ in 0..10 {
            flipped = next_day(&flipped);
        }
        assert_eq!(flipped.len(), 1373);

        // after day 90
        for _ in 0..10 {
            flipped = next_day(&flipped);
        }
        assert_eq!(flipped.len(), 1844);

        // after day 100
        for _ in 0..10 {
            flipped = next_day(&flipped);
        }
        assert_eq!(flipped.len(), 2208);
    }

    fn get_input() -> Vec<Tile> {
        vec![
            "sesenwnenenewseeswwswswwnenewsewsw",
            "neeenesenwnwwswnenewnwwsewnenwseswesw",
            "seswneswswsenwwnwse",
            "nwnwneseeswswnenewneswwnewseswneseene",
            "swweswneswnenwsewnwneneseenw",
            "eesenwseswswnenwswnwnwsewwnwsene",
            "sewnenenenesenwsewnenwwwse",
            "wenwwweseeeweswwwnwwe",
            "wsweesenenewnwwnwsenewsenwwsesesenwne",
            "neeswseenwwswnwswswnw",
            "nenwswwsewswnenenewsenwsenwnesesenew",
            "enewnwewneswsewnwswenweswnenwsenwsw",
            "sweneswneswneneenwnewenewwneswswnese",
            "swwesenesewenwneswnwwneseswwne",
            "enesenwswwswneneswsenwnewswseenwsese",
            "wnwnesenesenenwwnenwsewesewsesesew",
            "nenewswnwewswnenesenwnesewesw",
            "eneswnwswnwsenenwnwnwwseeswneewsenese",
            "neswnwewnwnwseenwseesewsenwsweewe",
            "wseweeenwnesenwwwswnew",
        ]
        .as_records::<Tile>()
        .unwrap()
    }

    fn get_expected(coords: &[(i32, i32)]) -> HashSet<Tile> {
        let mut expected = HashSet::new();
        expected.extend(coords.iter().map(|&c| Tile(c.0, c.1)));
        expected
    }
}
