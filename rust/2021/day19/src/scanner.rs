use std::collections::HashSet;
use std::fmt;

use crate::overlap::count_overlap;
use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::MultilineFromStr;
use rdcl_aoc_helpers::parse_error;

use crate::point::{Point, ORIENTATIONS};

const THRESHOLD: usize = 12;
const D_THRESHOLD: usize = THRESHOLD * (THRESHOLD - 1) / 2;

#[derive(Clone, Default, Debug)]
pub struct Scanner {
    pub idx: usize,
    pub orientation: u8,
    pub position: Point,
    pub beacons: HashSet<Point>,
    distances: Vec<i64>,
}

impl Scanner {
    pub fn adjust(&self, other: &Scanner) -> Option<Scanner> {
        if count_overlap(&self.distances, &other.distances) < D_THRESHOLD {
            // there is no way these scanners can have overlap, because the distances are too
            // different
            return None;
        }

        for &s_beacon in &self.beacons {
            for &o_beacon in &other.beacons {
                for orientation in 0..ORIENTATIONS {
                    let offset = s_beacon - o_beacon.rotate(orientation);
                    let scanner = other.transform(offset, orientation);
                    if self.count_overlap(&scanner) >= THRESHOLD {
                        return Some(scanner);
                    }
                }
            }
        }

        None
    }

    fn transform(&self, offset: Point, orientation: u8) -> Scanner {
        Scanner {
            idx: self.idx,
            position: offset,
            orientation,
            beacons: self
                .beacons
                .iter()
                .map(|p| p.rotate(orientation))
                .map(|p| p + offset)
                .collect(),
            distances: self.distances.clone(),
        }
    }

    fn count_overlap(&self, other: &Scanner) -> usize {
        self.beacons.intersection(&other.beacons).count()
    }
}

impl fmt::Display for Scanner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "--- scanner {} ---", self.idx)?;
        for point in &self.beacons {
            writeln!(f, "{}", point)?;
        }
        Ok(())
    }
}

impl MultilineFromStr for Scanner {
    type Err = ParseError;

    fn new() -> Self {
        Scanner::default()
    }

    fn indicates_new_record(&self, line: &str) -> bool {
        line.is_empty()
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        if line.is_empty() {
            Ok(())
        } else if let Some(sub) = line.strip_prefix("--- scanner ") {
            if let Some(sub) = sub.strip_suffix(" ---") {
                self.idx = sub.parse()?;
                Ok(())
            } else {
                Err(parse_error!("Invalid input line: '{}'", line))
            }
        } else {
            let beacon = line.parse::<Point>()?;
            for bo in &self.beacons {
                self.distances.push(beacon.distance_to(bo));
            }
            self.distances.sort_unstable();
            self.beacons.insert(beacon);
            Ok(())
        }
    }
}

#[cfg(test)]
pub mod tests {
    use rdcl_aoc_helpers::input::WithAsMultilineRecords;

    use super::*;

    #[test]
    fn test_adjust() {
        let scanners = test_scanners();

        // verify the correct number of distances are computed for the first scanner
        let n = scanners[0].beacons.len();
        assert_eq!(scanners[0].distances.len(), n * (n - 1) / 2);

        // verify the correct number of distances are computed for the second scanner
        let n = scanners[1].beacons.len();
        assert_eq!(scanners[1].distances.len(), n * (n - 1) / 2);

        // verify the second scanner can be adjusted to the first scanner
        let corrected = scanners[0].adjust(&scanners[1]);
        assert!(corrected.is_some());

        // verify the the second scanner ends up in the expected position
        let corrected = corrected.unwrap();
        assert_eq!(corrected.orientation, 20);
        assert_eq!(corrected.position, Point::new(68, -1246, -43));
    }

    pub fn test_scanners() -> Vec<Scanner> {
        vec![
            "--- scanner 0 ---",
            "404,-588,-901",
            "528,-643,409",
            "-838,591,734",
            "390,-675,-793",
            "-537,-823,-458",
            "-485,-357,347",
            "-345,-311,381",
            "-661,-816,-575",
            "-876,649,763",
            "-618,-824,-621",
            "553,345,-567",
            "474,580,667",
            "-447,-329,318",
            "-584,868,-557",
            "544,-627,-890",
            "564,392,-477",
            "455,729,728",
            "-892,524,684",
            "-689,845,-530",
            "423,-701,434",
            "7,-33,-71",
            "630,319,-379",
            "443,580,662",
            "-789,900,-551",
            "459,-707,401",
            "",
            "--- scanner 1 ---",
            "686,422,578",
            "605,423,415",
            "515,917,-361",
            "-336,658,858",
            "95,138,22",
            "-476,619,847",
            "-340,-569,-846",
            "567,-361,727",
            "-460,603,-452",
            "669,-402,600",
            "729,430,532",
            "-500,-761,534",
            "-322,571,750",
            "-466,-666,-811",
            "-429,-592,574",
            "-355,545,-477",
            "703,-491,-529",
            "-328,-685,520",
            "413,935,-424",
            "-391,539,-444",
            "586,-435,557",
            "-364,-763,-893",
            "807,-499,-711",
            "755,-354,-619",
            "553,889,-390",
            "",
            "--- scanner 2 ---",
            "649,640,665",
            "682,-795,504",
            "-784,533,-524",
            "-644,584,-595",
            "-588,-843,648",
            "-30,6,44",
            "-674,560,763",
            "500,723,-460",
            "609,671,-379",
            "-555,-800,653",
            "-675,-892,-343",
            "697,-426,-610",
            "578,704,681",
            "493,664,-388",
            "-671,-858,530",
            "-667,343,800",
            "571,-461,-707",
            "-138,-166,112",
            "-889,563,-600",
            "646,-828,498",
            "640,759,510",
            "-630,509,768",
            "-681,-892,-333",
            "673,-379,-804",
            "-742,-814,-386",
            "577,-820,562",
            "",
            "--- scanner 3 ---",
            "-589,542,597",
            "605,-692,669",
            "-500,565,-823",
            "-660,373,557",
            "-458,-679,-417",
            "-488,449,543",
            "-626,468,-788",
            "338,-750,-386",
            "528,-832,-391",
            "562,-778,733",
            "-938,-730,414",
            "543,643,-506",
            "-524,371,-870",
            "407,773,750",
            "-104,29,83",
            "378,-903,-323",
            "-778,-728,485",
            "426,699,580",
            "-438,-605,-362",
            "-469,-447,-387",
            "509,732,623",
            "647,635,-688",
            "-868,-804,481",
            "614,-800,639",
            "595,780,-596",
            "",
            "--- scanner 4 ---",
            "727,592,562",
            "-293,-554,779",
            "441,611,-461",
            "-714,465,-776",
            "-743,427,-804",
            "-660,-479,-426",
            "832,-632,460",
            "927,-485,-438",
            "408,393,-506",
            "466,436,-512",
            "110,16,151",
            "-258,-428,682",
            "-393,719,612",
            "-211,-452,876",
            "808,-476,-593",
            "-575,615,604",
            "-485,667,467",
            "-680,325,-822",
            "-627,-443,-432",
            "872,-547,-609",
            "833,512,582",
            "807,604,487",
            "839,-516,451",
            "891,-625,532",
            "-652,-548,-490",
            "30,-46,-14",
        ]
        .as_multiline_records()
        .unwrap()
    }
}
