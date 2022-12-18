use std::collections::HashSet;
use std::str::FromStr;

use rdcl_aoc_core::error::ParseError;

use crate::area::{Area, Plane};
use crate::point::Point;

#[derive(Debug, Clone)]
pub struct Region {
    pub surface: HashSet<Area>,
}

impl Region {
    pub fn new(point: Point) -> Region {
        Region {
            surface: HashSet::from([
                Area::new(Plane::X, point),
                Area::new(Plane::X, point + (1, 0, 0)),
                Area::new(Plane::Y, point),
                Area::new(Plane::Y, point + (0, 1, 0)),
                Area::new(Plane::Z, point),
                Area::new(Plane::Z, point + (0, 0, 1)),
            ]),
        }
    }

    pub fn join(&self, other: &Region) -> Option<Region> {
        if self.surface.is_disjoint(&other.surface) {
            None
        } else {
            let mut surface = HashSet::new();

            for area in &self.surface {
                if !other.surface.contains(area) {
                    surface.insert(*area);
                }
            }

            for area in &other.surface {
                if !self.surface.contains(area) {
                    surface.insert(*area);
                }
            }

            Some(Region { surface })
        }
    }

    pub fn surface_area(&self) -> usize {
        self.surface.len()
    }

    pub fn can_reach(&self, p1: Point, p2: Point) -> bool {
        if p1.x + 1 == p2.x {
            !self.surface.contains(&Area::new(Plane::X, p2))
        } else if p1.x == p2.x + 1 {
            !self.surface.contains(&Area::new(Plane::X, p1))
        } else if p1.y + 1 == p2.y {
            !self.surface.contains(&Area::new(Plane::Y, p2))
        } else if p1.y == p2.y + 1 {
            !self.surface.contains(&Area::new(Plane::Y, p1))
        } else if p1.z + 1 == p2.z {
            !self.surface.contains(&Area::new(Plane::Z, p2))
        } else if p1.z == p2.z + 1 {
            !self.surface.contains(&Area::new(Plane::Z, p1))
        } else {
            false
        }
    }
}

impl FromStr for Region {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Region::new(s.parse()?))
    }
}
