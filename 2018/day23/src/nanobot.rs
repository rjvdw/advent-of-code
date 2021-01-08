use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::math::taxi_cab_3d;

use crate::region::Region;
use crate::Point;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Nanobot {
    pub position: Point,
    pub radius: i64,
}

impl Nanobot {
    /// Checks whether a point is in our sphere of influence.
    pub fn in_range(&self, (x, y, z): Point) -> bool {
        taxi_cab_3d(self.position, (x, y, z)) <= self.radius
    }

    /// Returns true if the nanobot can influence some point within the region.
    pub fn influences(&self, region: &Region) -> bool {
        let (x, y, z) = self.position;
        let r = self.radius;

        let points_of_interest = vec![
            (x, y, z),
            (x + r, y, z),
            (x - r, y, z),
            (x, y + r, z),
            (x, y - r, z),
            (x, y, z + r),
            (x, y, z - r),
        ];

        points_of_interest.iter().any(|&p| region.contains(p))
            || region.get_corners().iter().any(|&p| self.in_range(p))
    }

    /// Returns true if the nanobot can influence all points within the region.
    pub fn all_in_range(&self, region: &Region) -> bool {
        region.get_corners().iter().all(|&p| self.in_range(p))
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
