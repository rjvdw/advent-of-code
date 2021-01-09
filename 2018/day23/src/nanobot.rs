use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::math::taxi_cab_3d;

use crate::region::Region;
use crate::{check_if_edge_overlaps, Point};

/// A single nanobot.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Nanobot {
    /// The position in space of the nanobot.
    pub position: Point,

    /// The radius of its sphere of influence.
    pub radius: i64,
}

impl Nanobot {
    /// Checks whether a point is in our sphere of influence.
    pub fn in_range(&self, (x, y, z): Point) -> bool {
        taxi_cab_3d(self.position, (x, y, z)) <= self.radius
    }

    /// Returns true if the nanobot can influence some point within the region.
    pub fn influences(&self, region: &Region) -> bool {
        // if a corner of the sphere of influence overlaps with the region ...
        if self.get_corners().iter().any(|&p| region.contains(p)) {
            return true;
        }

        // if a corner of the region falls within the sphere of influence ...
        if region.get_corners().iter().any(|&p| self.in_range(p)) {
            return true;
        }

        // if the edges overlap (x, y and z) ...
        if check_if_edge_overlaps!(self, region, 0) {
            return true;
        }
        if check_if_edge_overlaps!(self, region, 1) {
            return true;
        }
        if check_if_edge_overlaps!(self, region, 2) {
            return true;
        }

        false
    }

    /// Returns the corners of the sphere of influence of the nanobot.
    fn get_corners(&self) -> Vec<Point> {
        let (x, y, z) = self.position;
        let r = self.radius;

        vec![
            (x + r, y, z),
            (x - r, y, z),
            (x, y + r, z),
            (x, y - r, z),
            (x, y, z + r),
            (x, y, z - r),
        ]
    }
}

impl fmt::Display for Nanobot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pos=<{},{},{}>, r={}",
            self.position.0, self.position.1, self.position.2, self.radius
        )
    }
}

impl FromStr for Nanobot {
    type Err = ParseError;

    fn from_str(nanobot: &str) -> Result<Self, Self::Err> {
        let mut pos = (String::new(), String::new(), String::new());
        let mut r = String::new();
        let mut s = ParsingState::P1;

        for ch in nanobot.chars() {
            match s {
                ParsingState::P1 => {
                    if ch == '-' || ch.is_numeric() {
                        pos.0.push(ch);
                    } else if ch == ',' {
                        s = ParsingState::P2;
                    }
                }
                ParsingState::P2 => {
                    if ch == '-' || ch.is_numeric() {
                        pos.1.push(ch);
                    } else if ch == ',' {
                        s = ParsingState::P3;
                    }
                }
                ParsingState::P3 => {
                    if ch == '-' || ch.is_numeric() {
                        pos.2.push(ch);
                    } else if ch == ',' {
                        s = ParsingState::R;
                    }
                }
                ParsingState::R => {
                    if ch == '-' || ch.is_numeric() {
                        r.push(ch);
                    }
                }
            }
        }

        Ok(Nanobot {
            position: (pos.0.parse()?, pos.1.parse()?, pos.2.parse()?),
            radius: r.parse()?,
        })
    }
}

enum ParsingState {
    P1,
    P2,
    P3,
    R,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nanobot_influences_region_1() {
        // corner of the nanobot falls within the region

        let nanobot = Nanobot {
            position: (7, 0, 0),
            radius: 5,
        };
        let region = Region {
            from: (-3, -3, -3),
            to: (3, 3, 3),
        };

        assert!(nanobot.influences(&region));
    }

    #[test]
    fn test_nanobot_influences_region_2() {
        // corner of the region falls within the sphere of influence

        let nanobot = Nanobot {
            position: (0, 0, 0),
            radius: 5,
        };
        let region = Region {
            from: (1, 1, 1),
            to: (7, 7, 7),
        };

        assert!(nanobot.influences(&region));
    }

    #[test]
    fn test_nanobot_influences_region_3() {
        // edge of the region falls within the sphere of influence, but all corners are disjoint

        let nanobot = Nanobot {
            position: (0, 0, 0),
            radius: 5,
        };
        let region = Region {
            from: (2, 2, -3),
            to: (8, 8, 3),
        };

        assert!(nanobot.influences(&region));
    }

    #[test]
    fn test_corners_are_in_range() {
        let nb = Nanobot {
            position: (0, 0, 0),
            radius: 3,
        };
        let corners = nb.get_corners();

        assert_eq!(corners.len(), 6);

        assert_eq!(corners[0], (3, 0, 0));
        assert_eq!(corners[1], (-3, 0, 0));
        assert_eq!(corners[2], (0, 3, 0));
        assert_eq!(corners[3], (0, -3, 0));
        assert_eq!(corners[4], (0, 0, 3));
        assert_eq!(corners[5], (0, 0, -3));

        assert!(corners.iter().all(|&c| nb.in_range(c)));
    }
}
