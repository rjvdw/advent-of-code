extern crate rdcl_aoc_helpers;

use std::collections::HashMap;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadMultiLines;

use crate::dragon::{DRAGON_IMAGE, DRAGON_IMAGE_WIDTH, NR_ACTIVE_PIXELS_IN_DRAGON};
use crate::edges::EdgeName;
use crate::orientation_helpers::orient_x_y;
use crate::tile::{OrientedTile, Tile, TilesById, SIZE};

mod dragon;
mod edges;
mod orientation_helpers;
mod tile;

type Grid<T> = Vec<Vec<T>>;

/// https://adventofcode.com/2020/day/20
fn main() {
    let args = get_args(&["<input file>"], 1);

    let tiles = File::open(&args[1])
        .read_multi_lines(1)
        .skip(1)
        .collect::<Vec<Tile>>();
    let tiles = make_map(&tiles);
    let corner_tiles = find_corner_tiles(&tiles);

    println!(
        "The product of the four corner tiles is: {}",
        corner_tiles.iter().product::<u64>()
    );

    let composed = compose(&tiles, *corner_tiles.iter().min().unwrap());
    let image = compile_image(&tiles, &composed);
    let nr_dragons = count_dragons(&image);

    println!(
        "The roughness of the water is {}",
        image.chars().filter(|&ch| ch == '#').count() - nr_dragons * NR_ACTIVE_PIXELS_IN_DRAGON
    );
}

fn make_map(tiles: &[Tile]) -> TilesById {
    tiles.iter().fold(HashMap::new(), |mut map, &tile| {
        map.insert(tile.id, tile);
        map
    })
}

fn find_corner_tiles(tiles: &TilesById) -> Vec<u64> {
    let mut corner_tiles = Vec::new();

    for tile in tiles.values() {
        if tile.find_adjecent_tiles(tiles).len() == 2 {
            corner_tiles.push(tile.id);
        }
    }

    corner_tiles
}

fn compose(tiles: &TilesById, start_from: u64) -> Grid<OrientedTile> {
    let mut at = (start_from, get_starting_orientation(tiles, start_from));
    let mut row_start = at;
    let mut composed = Vec::new();
    let mut row = vec![at];

    loop {
        match get_next_tile(tiles, at, EdgeName::Right) {
            Some(next) => {
                at = next;
            }
            None => {
                let next_row = row.to_vec();
                row = Vec::new();
                composed.push(next_row);
                if let Some(next) = get_next_tile(tiles, row_start, EdgeName::Bottom) {
                    at = next;
                    row_start = next;
                } else {
                    break;
                }
            }
        };
        row.push(at);
    }

    composed
}

fn get_next_tile(
    tiles: &TilesById,
    (id, orientation): OrientedTile,
    edge_name: EdgeName,
) -> Option<OrientedTile> {
    tiles
        .get(&id)
        .unwrap()
        .orient(orientation)
        .find_adjacent_tiles_to_edge(tiles, edge_name)
}

fn get_starting_orientation(tiles: &TilesById, start_from: u64) -> u8 {
    let tile = tiles.get(&start_from).unwrap();
    let top = tile.find_adjacent_tiles_to_edge(tiles, EdgeName::Top);
    let right = tile.find_adjacent_tiles_to_edge(tiles, EdgeName::Right);

    match (top, right) {
        (None, None) => 0b001,
        (Some(_), None) => 0b010,
        (None, Some(_)) => 0b000,
        (Some(_), Some(_)) => 0b011,
    }
}

fn compile_image(tiles: &TilesById, composed: &[Vec<OrientedTile>]) -> String {
    let mut image = String::new();

    for tiles_row in composed {
        for row in 1..SIZE - 1 {
            if !image.is_empty() {
                image.push('\n');
            }
            for &(id, orientation) in tiles_row {
                let tile = *tiles.get(&id).unwrap();
                let tile = tile.orient(orientation);

                for column in 1..SIZE - 1 {
                    if tile.at(row, column) {
                        image.push('#');
                    } else {
                        image.push('.');
                    }
                }
            }
        }
    }

    image
}

fn count_dragons(image: &str) -> usize {
    let mut nr_dragons = 0;

    let image_lines = image
        .split('\n')
        .map(parse_image_line)
        .collect::<Vec<u128>>();

    // assumption: the image is square
    let image_size = image_lines.len();

    let dragon_lines = DRAGON_IMAGE
        .iter()
        .map(|&line| parse_image_line(line))
        .collect::<Vec<u128>>();

    for orientation in 0b000..0b111 {
        let image_lines = orient(&image_lines, orientation, image_size);

        for (row, &line2) in image_lines.iter().enumerate().skip(2) {
            let line0 = image_lines[row - 2];
            let line1 = image_lines[row - 1];

            for column in 0..image_size - DRAGON_IMAGE_WIDTH {
                let dragon_line0 = dragon_lines[0] << column;
                let dragon_line1 = dragon_lines[1] << column;
                let dragon_line2 = dragon_lines[2] << column;

                if (line0 | dragon_line0 == line0)
                    && (line1 | dragon_line1 == line1)
                    && (line2 | dragon_line2 == line2)
                {
                    nr_dragons += 1;
                }
            }
        }
    }

    nr_dragons
}

fn orient(image: &[u128], orientation: u8, size: usize) -> Vec<u128> {
    let mut oriented = Vec::new();

    for row in 0..size {
        let mut oriented_row = 0u128;
        for column in 0..size {
            oriented_row <<= 1;
            let (row, column) = orient_x_y(row, column, size, orientation);
            if image[row] & (1 << (size - 1 - column)) != 0 {
                oriented_row |= 1;
            }
        }
        oriented.push(oriented_row);
    }

    oriented
}

fn parse_image_line(line: &str) -> u128 {
    let mut parsed = 0;

    for ch in line.chars() {
        parsed <<= 1;
        if ch == '#' {
            parsed |= 1;
        }
    }

    parsed
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsMultilineRecords;

    use super::*;

    #[test]
    fn test_find_corner_tiles() {
        let tiles = get_test_input();

        assert_eq!(
            find_corner_tiles(&tiles).iter().product::<u64>(),
            20899048083289
        );
    }

    #[test]
    fn test_compose() {
        let mut tiles = get_test_input();
        // flip tile 1951, so that the output will match the output on adventofcode.com
        tiles.insert(1951, tiles.get(&1951).unwrap().orient(0b100));
        let composed = compose(&tiles, 1951);

        assert_eq!(
            composed,
            vec![
                vec![(1951, 1), (2311, 5), (3079, 0)],
                vec![(2729, 5), (1427, 5), (2473, 6)],
                vec![(2971, 5), (1489, 5), (1171, 7)],
            ]
        );
    }

    #[test]
    fn test_compile_image() {
        let (tiles, composed) = get_test_input_for_part_2();
        let image = compile_image(&tiles, &composed);

        assert_eq!(
            image.split('\n').collect::<Vec<&str>>(),
            vec![
                ".#.#..#.##...#.##..#####",
                "###....#.#....#..#......",
                "##.##.###.#.#..######...",
                "###.#####...#.#####.#..#",
                "##.#....#.##.####...#.##",
                "...########.#....#####.#",
                "....#..#...##..#.#.###..",
                ".####...#..#.....#......",
                "#..#.##..#..###.#.##....",
                "#.####..#.####.#.#.###..",
                "###.#.#...#.######.#..##",
                "#.####....##..########.#",
                "##..##.#...#...#.#.#.#..",
                "...#..#..#.#.##..###.###",
                ".#.#....#.##.#...###.##.",
                "###.#...#..#.##.######..",
                ".#.#.###.##.##.#..#.##..",
                ".####.###.#...###.#..#.#",
                "..#.#..#..#.#.#.####.###",
                "#..####...#.#.#.###.###.",
                "#####..#####...###....##",
                "#.##..#..#...#..####...#",
                ".#.###..##..##..####.##.",
                "...###...##...#...#..###",
            ]
        );
    }

    #[test]
    fn test_count_dragons() {
        let (tiles, composed) = get_test_input_for_part_2();
        let image = compile_image(&tiles, &composed);
        let nr_dragons = count_dragons(&image);

        assert_eq!(nr_dragons, 2);

        let nr_pixels =
            image.chars().filter(|&ch| ch == '#').count() - nr_dragons * NR_ACTIVE_PIXELS_IN_DRAGON;

        assert_eq!(nr_pixels, 273);

        // TODO: Would be nice if we could create this image:
        // let image = mark_dragons(&image);
        //
        // assert_eq!(
        //     image.split('\n').collect::<Vec<&str>>(),
        //     vec![
        //         ".####...#####..#...###..",
        //         "#####..#..#.#.####..#.#.",
        //         ".#.#...#.###...#.##.O#..",
        //         "#.O.##.OO#.#.OO.##.OOO##",
        //         "..#O.#O#.O##O..O.#O##.##",
        //         "...#.#..##.##...#..#..##",
        //         "#.##.#..#.#..#..##.#.#..",
        //         ".###.##.....#...###.#...",
        //         "#.####.#.#....##.#..#.#.",
        //         "##...#..#....#..#...####",
        //         "..#.##...###..#.#####..#",
        //         "....#.##.#.#####....#...",
        //         "..##.##.###.....#.##..#.",
        //         "#...#...###..####....##.",
        //         ".#.##...#.##.#.#.###...#",
        //         "#.###.#..####...##..#...",
        //         "#.###...#.##...#.##O###.",
        //         ".O##.#OO.###OO##..OOO##.",
        //         "..O#.O..O..O.#O##O##.###",
        //         "#.#..##.########..#..##.",
        //         "#.#####..#.#...##..#....",
        //         "#....##..#.#########..##",
        //         "#...#.....#..##...###.##",
        //         "#..###....##.#...##.##.#",
        //     ]
        // );
    }

    fn get_test_input() -> TilesById {
        let tiles: Vec<Tile> = vec![
            "Tile 2311:",
            "..##.#..#.",
            "##..#.....",
            "#...##..#.",
            "####.#...#",
            "##.##.###.",
            "##...#.###",
            ".#.#.#..##",
            "..#....#..",
            "###...#.#.",
            "..###..###",
            "",
            "Tile 1951:",
            "#.##...##.",
            "#.####...#",
            ".....#..##",
            "#...######",
            ".##.#....#",
            ".###.#####",
            "###.##.##.",
            ".###....#.",
            "..#.#..#.#",
            "#...##.#..",
            "",
            "Tile 1171:",
            "####...##.",
            "#..##.#..#",
            "##.#..#.#.",
            ".###.####.",
            "..###.####",
            ".##....##.",
            ".#...####.",
            "#.##.####.",
            "####..#...",
            ".....##...",
            "",
            "Tile 1427:",
            "###.##.#..",
            ".#..#.##..",
            ".#.##.#..#",
            "#.#.#.##.#",
            "....#...##",
            "...##..##.",
            "...#.#####",
            ".#.####.#.",
            "..#..###.#",
            "..##.#..#.",
            "",
            "Tile 1489:",
            "##.#.#....",
            "..##...#..",
            ".##..##...",
            "..#...#...",
            "#####...#.",
            "#..#.#.#.#",
            "...#.#.#..",
            "##.#...##.",
            "..##.##.##",
            "###.##.#..",
            "",
            "Tile 2473:",
            "#....####.",
            "#..#.##...",
            "#.##..#...",
            "######.#.#",
            ".#...#.#.#",
            ".#########",
            ".###.#..#.",
            "########.#",
            "##...##.#.",
            "..###.#.#.",
            "",
            "Tile 2971:",
            "..#.#....#",
            "#...###...",
            "#.#.###...",
            "##.##..#..",
            ".#####..##",
            ".#..####.#",
            "#..#.#..#.",
            "..####.###",
            "..#.#.###.",
            "...#.#.#.#",
            "",
            "Tile 2729:",
            "...#.#.#.#",
            "####.#....",
            "..#.#.....",
            "....#..#.#",
            ".##..##.#.",
            ".#.####...",
            "####.#.#..",
            "##.####...",
            "##..#.##..",
            "#.##...##.",
            "",
            "Tile 3079:",
            "#.#.#####.",
            ".#..######",
            "..#.......",
            "######....",
            "####.#..#.",
            ".#...#.##.",
            "#.#####.##",
            "..#.###...",
            "..#.......",
            "..#.###...",
        ]
        .as_multiline_records()
        .unwrap()
        .iter()
        .skip(1)
        .cloned()
        .collect();

        make_map(&tiles)
    }

    fn get_test_input_for_part_2() -> (TilesById, Grid<OrientedTile>) {
        let mut tiles = get_test_input();
        // flip tile 1951, so that the output will match the output on adventofcode.com
        tiles.insert(1951, tiles.get(&1951).unwrap().orient(0b100));
        let composed = compose(&tiles, 1951);

        (tiles, composed)
    }
}
