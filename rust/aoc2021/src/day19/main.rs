extern crate rdcl_aoc_helpers;

use std::collections::HashSet;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadMultiLines;

use crate::point::Point;
use crate::scanner::Scanner;

mod overlap;
mod point;
mod scanner;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let scanners = File::open(&args[1])
        .read_multi_lines(1)
        .collect::<Vec<Scanner>>();

    let scanners = correct_scanners(&scanners);
    let beacons = find_beacons(&scanners);
    println!("There are {} beacons.", beacons.len());
    println!(
        "The greatest distance between scanners is {}.",
        max_distance(&scanners)
    );
}

fn correct_scanners(scanners: &[Scanner]) -> Vec<Scanner> {
    let mut i = 0;
    let mut handled = HashSet::new();
    handled.insert(i);
    let mut corrected = vec![scanners[i].clone()];

    while corrected.len() < scanners.len() {
        for scanner in scanners {
            if handled.contains(&scanner.idx) {
                continue;
            }
            if let Some(scanner) = corrected[i].adjust(scanner) {
                handled.insert(scanner.idx);
                corrected.push(scanner);
            }
        }
        i += 1;
    }

    corrected
}

fn find_beacons(scanners: &[Scanner]) -> HashSet<Point> {
    scanners
        .iter()
        .flat_map(|scanner| scanner.beacons.iter())
        .copied()
        .collect()
}

fn max_distance(scanners: &[Scanner]) -> u64 {
    let mut max = u64::MIN;
    for p1 in scanners {
        for p2 in scanners {
            let distance = p1.position.distance_to(&p2.position) as u64;
            if distance > max {
                max = distance;
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use crate::scanner::tests::test_scanners;

    use super::*;

    #[test]
    fn test_correct_scanners() {
        let scanners = test_scanners();
        let corrected = correct_scanners(&scanners);
        let mut beacons = find_beacons(&corrected)
            .iter()
            .copied()
            .collect::<Vec<Point>>();
        beacons.sort_unstable();
        assert_eq!(beacons, all_beacons());
    }

    #[test]
    fn test_max_distance() {
        let scanners = test_scanners();
        let corrected = correct_scanners(&scanners);
        assert_eq!(max_distance(&corrected), 3621);
    }

    fn all_beacons() -> Vec<Point> {
        let mut b = vec![
            Point::new(-892, 524, 684),
            Point::new(-876, 649, 763),
            Point::new(-838, 591, 734),
            Point::new(-789, 900, -551),
            Point::new(-739, -1745, 668),
            Point::new(-706, -3180, -659),
            Point::new(-697, -3072, -689),
            Point::new(-689, 845, -530),
            Point::new(-687, -1600, 576),
            Point::new(-661, -816, -575),
            Point::new(-654, -3158, -753),
            Point::new(-635, -1737, 486),
            Point::new(-631, -672, 1502),
            Point::new(-624, -1620, 1868),
            Point::new(-620, -3212, 371),
            Point::new(-618, -824, -621),
            Point::new(-612, -1695, 1788),
            Point::new(-601, -1648, -643),
            Point::new(-584, 868, -557),
            Point::new(-537, -823, -458),
            Point::new(-532, -1715, 1894),
            Point::new(-518, -1681, -600),
            Point::new(-499, -1607, -770),
            Point::new(-485, -357, 347),
            Point::new(-470, -3283, 303),
            Point::new(-456, -621, 1527),
            Point::new(-447, -329, 318),
            Point::new(-430, -3130, 366),
            Point::new(-413, -627, 1469),
            Point::new(-345, -311, 381),
            Point::new(-36, -1284, 1171),
            Point::new(-27, -1108, -65),
            Point::new(7, -33, -71),
            Point::new(12, -2351, -103),
            Point::new(26, -1119, 1091),
            Point::new(346, -2985, 342),
            Point::new(366, -3059, 397),
            Point::new(377, -2827, 367),
            Point::new(390, -675, -793),
            Point::new(396, -1931, -563),
            Point::new(404, -588, -901),
            Point::new(408, -1815, 803),
            Point::new(423, -701, 434),
            Point::new(432, -2009, 850),
            Point::new(443, 580, 662),
            Point::new(455, 729, 728),
            Point::new(456, -540, 1869),
            Point::new(459, -707, 401),
            Point::new(465, -695, 1988),
            Point::new(474, 580, 667),
            Point::new(496, -1584, 1900),
            Point::new(497, -1838, -617),
            Point::new(527, -524, 1933),
            Point::new(528, -643, 409),
            Point::new(534, -1912, 768),
            Point::new(544, -627, -890),
            Point::new(553, 345, -567),
            Point::new(564, 392, -477),
            Point::new(568, -2007, -577),
            Point::new(605, -1665, 1952),
            Point::new(612, -1593, 1893),
            Point::new(630, 319, -379),
            Point::new(686, -3108, -505),
            Point::new(776, -3184, -501),
            Point::new(846, -3110, -434),
            Point::new(1135, -1161, 1235),
            Point::new(1243, -1093, 1063),
            Point::new(1660, -552, 429),
            Point::new(1693, -557, 386),
            Point::new(1735, -437, 1738),
            Point::new(1749, -1800, 1813),
            Point::new(1772, -405, 1572),
            Point::new(1776, -675, 371),
            Point::new(1779, -442, 1789),
            Point::new(1780, -1548, 337),
            Point::new(1786, -1538, 337),
            Point::new(1847, -1591, 415),
            Point::new(1889, -1729, 1762),
            Point::new(1994, -1805, 1792),
        ];
        b.sort_unstable();
        b
    }
}
