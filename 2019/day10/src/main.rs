use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};

use crate::i2d_range::I2dRange;

mod i2d_range;

type AsteroidMap = HashSet<(i64, i64)>;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let asteroid_map = parse_input(file).or_exit_with(1);

    let (location, count) = find_best_location(&asteroid_map);
    println!(
        "From the best location ({:?}), you can detect {} asteroids.",
        location, count
    );

    let vaporized = vaporize_asteroids(asteroid_map, location);
    println!(
        "The 200th asteroid to be vaporized is at {:?} (answer={}).",
        vaporized[199],
        vaporized[199].0 * 100 + vaporized[199].1
    );
}

fn find_best_location(asteroid_map: &AsteroidMap) -> ((i64, i64), usize) {
    let mut best_location = ((-1, -1), usize::MIN);
    for &p1 in asteroid_map {
        let mut count = 0;
        for &p2 in asteroid_map {
            if p1 != p2 && is_visible(asteroid_map, p1, p2) {
                count += 1;
            }
        }
        if count > best_location.1 {
            best_location = (p1, count);
        }
    }
    best_location
}

fn vaporize_asteroids(mut asteroid_map: AsteroidMap, location: (i64, i64)) -> Vec<(i64, i64)> {
    let mut vaporized = vec![];

    // We remove our current location from the map, as we are not going to vaporize this asteroid.
    asteroid_map.remove(&location);

    // The laser will keep doing passes until there are no asteroids left.
    while !asteroid_map.is_empty() {
        // Start by checking which asteroids will be visible in this current pass.
        let mut visible: Vec<(i64, i64)> = asteroid_map
            .iter()
            .copied()
            .filter(|&p| is_visible(&asteroid_map, location, p))
            .collect();

        // We sort the points that are currently visible from the origin:
        // * First by sector.
        // * Then, if the two points are in the samen sector, by comparing the normalized points.
        // (See the docs of the used functions for more details.)
        visible.sort_unstable_by(|&p1, &p2| {
            let p1 = (p1.0 - location.0, p1.1 - location.1);
            let p2 = (p2.0 - location.0, p2.1 - location.1);
            let sector = determine_sector(p1);
            sector.cmp(&determine_sector(p2)).then_with(|| {
                let p1n = normalize(sector, p1);
                let p2n = normalize(sector, p2);
                compare_normalized(p1n, p2n)
            })
        });

        // Vaporize the asteroids in the specified order.
        for asteroid in visible {
            asteroid_map.remove(&asteroid);
            vaporized.push(asteroid);
        }
    }

    vaporized
}

/// Since the laser starts pointing straight up, and then moves clockwise, it is helpful to split
/// the space into four sectors. This method determines in which sector the point is.
/// - 0: Top-right (including the left border)
/// - 1: Bottom-right (including the top border)
/// - 2: Bottom-left (including the right border)
/// - 3: Top-left (including the bottom border)
///
/// Will panic if it receives (0, 0), as this point is not in any sector.
///
/// Visually:
/// ```
///      |
///   3  |  0
///      |
/// -----o-----
///      |
///   2  |  1
///      |
/// ```
fn determine_sector((x, y): (i64, i64)) -> i64 {
    match (x.cmp(&0), y.cmp(&0)) {
        // origin may not appear
        (Ordering::Equal, Ordering::Equal) => unreachable!(),
        // x >= 0 && y < 0
        (Ordering::Greater, Ordering::Less) => 0,
        (Ordering::Equal, Ordering::Less) => 0,
        // x > 0 && y >= 0
        (Ordering::Greater, Ordering::Greater) => 1,
        (Ordering::Greater, Ordering::Equal) => 1,
        // x <= 0 && y > 0
        (Ordering::Less, Ordering::Greater) => 2,
        (Ordering::Equal, Ordering::Greater) => 2,
        // x < 0 && y <= 0
        (Ordering::Less, Ordering::Less) => 3,
        (Ordering::Less, Ordering::Equal) => 3,
    }
}

/// Given the sector, normalize the provided point. Basically: Rotate the sector so that it lines up
/// with sector 0. This allows us to simplify the comparison between points that are inside the same
/// sector, as we only have to write a check for one single sector.
fn normalize(sector: i64, (x, y): (i64, i64)) -> (i64, i64) {
    match sector {
        0 => (x, y),
        1 => (-y, x),
        2 => (-x, -y),
        3 => (y, -x),
        _ => unreachable!(),
    }
}

/// Compare two normalized points (i.e. rotated around the origin, such that they end up in sector
/// 0). We compare by doing:
/// ```
/// |x1|   |x2|
/// ---- < ----
/// |y1|   |y2|
/// ```
/// Which translates to: `|x1| |y2| < |x2| |y1|`.
fn compare_normalized((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> Ordering {
    (x1.abs() * y2.abs()).cmp(&(x2.abs() * y1.abs()))
}

/// Checks if an asteroid is visible from a specified location, or if it is blocked by some other
/// asteroid.
fn is_visible(asteroid_map: &AsteroidMap, from: (i64, i64), target: (i64, i64)) -> bool {
    for point in I2dRange::new(from, target) {
        if asteroid_map.contains(&point) {
            return false;
        }
    }
    true
}

/// Parse the provided input into a HashSet containing all the locations that have an asteroid.
fn parse_input<R: Read>(r: R) -> Result<AsteroidMap, ParseError> {
    let mut map: AsteroidMap = HashSet::new();
    for (y, line) in BufReader::new(r).lines().enumerate() {
        let line = line?;
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                map.insert((x as i64, y as i64));
            }
        }
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip::macros(vec)]
    fn test_find_best_location_1() {
        let test_input = vec![
            ".#..#",
            ".....",
            "#####",
            "....#",
            "...##",
        ];
        let map = parse_input(test_input.join("\n").as_bytes()).unwrap();
        assert_eq!(find_best_location(&map), ((3, 4), 8));
    }

    #[test]
    fn test_find_best_location_2() {
        let test_input = vec![
            "......#.#.",
            "#..#.#....",
            "..#######.",
            ".#.#.###..",
            ".#..#.....",
            "..#....#.#",
            "#..#....#.",
            ".##.#..###",
            "##...#..#.",
            ".#....####",
        ];
        let map = parse_input(test_input.join("\n").as_bytes()).unwrap();
        assert_eq!(find_best_location(&map), ((5, 8), 33));
    }

    #[test]
    fn test_find_best_location_3() {
        let test_input = vec![
            "#.#...#.#.",
            ".###....#.",
            ".#....#...",
            "##.#.#.#.#",
            "....#.#.#.",
            ".##..###.#",
            "..#...##..",
            "..##....##",
            "......#...",
            ".####.###.",
        ];
        let map = parse_input(test_input.join("\n").as_bytes()).unwrap();
        assert_eq!(find_best_location(&map), ((1, 2), 35));
    }

    #[test]
    fn test_find_best_location_4() {
        let test_input = vec![
            ".#..#..###",
            "####.###.#",
            "....###.#.",
            "..###.##.#",
            "##.##.#.#.",
            "....###..#",
            "..#.#..#.#",
            "#..#.#.###",
            ".##...##.#",
            ".....#.#..",
        ];
        let map = parse_input(test_input.join("\n").as_bytes()).unwrap();
        assert_eq!(find_best_location(&map), ((6, 3), 41));
    }

    #[test]
    fn test_find_best_location_5() {
        let test_input = vec![
            ".#..##.###...#######",
            "##.############..##.",
            ".#.######.########.#",
            ".###.#######.####.#.",
            "#####.##.#.##.###.##",
            "..#####..#.#########",
            "####################",
            "#.####....###.#.#.##",
            "##.#################",
            "#####.##.###..####..",
            "..######..##.#######",
            "####.##.####...##..#",
            ".#####..#.######.###",
            "##...#.##########...",
            "#.##########.#######",
            ".####.#.###.###.#.##",
            "....##.##.###..#####",
            ".#.#.###########.###",
            "#.#.#.#####.####.###",
            "###.##.####.##.#..##",
        ];
        let map = parse_input(test_input.join("\n").as_bytes()).unwrap();
        assert_eq!(find_best_location(&map), ((11, 13), 210));
    }

    #[test]
    fn test_vaporize_asteroids_1() {
        let test_input = vec![
            ".#....#####...#..",
            "##...##.#####..##",
            "##...#...#.#####.",
            "..#.....#...###..",
            "..#.#.....#....##",
        ];
        let map = parse_input(test_input.join("\n").as_bytes()).unwrap();
        assert_eq!(
            vaporize_asteroids(map, (8, 3)),
            vec![
                (8, 1),
                (9, 0),
                (9, 1),
                (10, 0),
                (9, 2),
                (11, 1),
                (12, 1),
                (11, 2),
                (15, 1),
                (12, 2),
                (13, 2),
                (14, 2),
                (15, 2),
                (12, 3),
                (16, 4),
                (15, 4),
                (10, 4),
                (4, 4),
                (2, 4),
                (2, 3),
                (0, 2),
                (1, 2),
                (0, 1),
                (1, 1),
                (5, 2),
                (1, 0),
                (5, 1),
                (6, 1),
                (6, 0),
                (7, 0),
                (8, 0),
                (10, 1),
                (14, 0),
                (16, 1),
                (13, 3),
                (14, 3)
            ]
        )
    }

    #[test]
    fn test_vaporize_asteroids_2() {
        let test_input = vec![
            ".#..##.###...#######",
            "##.############..##.",
            ".#.######.########.#",
            ".###.#######.####.#.",
            "#####.##.#.##.###.##",
            "..#####..#.#########",
            "####################",
            "#.####....###.#.#.##",
            "##.#################",
            "#####.##.###..####..",
            "..######..##.#######",
            "####.##.####...##..#",
            ".#####..#.######.###",
            "##...#.##########...",
            "#.##########.#######",
            ".####.#.###.###.#.##",
            "....##.##.###..#####",
            ".#.#.###########.###",
            "#.#.#.#####.####.###",
            "###.##.####.##.#..##",
        ];
        let map = parse_input(test_input.join("\n").as_bytes()).unwrap();
        let vaporized = vaporize_asteroids(map, (11, 13));
        assert_eq!(vaporized[0], (11, 12));
        assert_eq!(vaporized[1], (12, 1));
        assert_eq!(vaporized[2], (12, 2));
        assert_eq!(vaporized[9], (12, 8));
        assert_eq!(vaporized[19], (16, 0));
        assert_eq!(vaporized[49], (16, 9));
        assert_eq!(vaporized[99], (10, 16));
        assert_eq!(vaporized[198], (9, 6));
        assert_eq!(vaporized[199], (8, 2));
        assert_eq!(vaporized[200], (10, 9));
        assert_eq!(vaporized[298], (11, 1));
    }
}
