use std::fmt;
use std::ops::Sub;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

use crate::range::Range;

#[derive(Copy, Clone)]
pub struct Cuboid {
    pub is_on: bool,
    pub x_range: Range,
    pub y_range: Range,
    pub z_range: Range,
}

impl Cuboid {
    pub fn is_disjoint(&self, other: &Cuboid) -> bool {
        self.x_range.is_disjoint(&other.x_range)
            || self.y_range.is_disjoint(&other.y_range)
            || self.z_range.is_disjoint(&other.z_range)
    }

    pub fn fits_within(&self, other: &Cuboid) -> bool {
        self.x_range.fits_within(&other.x_range)
            && self.y_range.fits_within(&other.y_range)
            && self.z_range.fits_within(&other.z_range)
    }

    pub fn size(&self) -> usize {
        self.x_range.size() * self.y_range.size() * self.z_range.size()
    }
}

impl<'a> Sub<&'a Cuboid> for &'a Cuboid {
    type Output = Vec<Cuboid>;

    /// Return a selection of cuboids that together contain all points that are in the first cuboid,
    /// but not in the second.
    fn sub(self, other: &Cuboid) -> Self::Output {
        self.x_range
            .partition(&other.x_range)
            .iter()
            .flat_map(|x_range| {
                self.y_range
                    .partition(&other.y_range)
                    .iter()
                    .map(|y_range| (*x_range, *y_range))
                    .collect::<Vec<(Range, Range)>>()
            })
            .flat_map(|(x_range, y_range)| {
                self.z_range
                    .partition(&other.z_range)
                    .iter()
                    .map(|z_range| (x_range, y_range, *z_range))
                    .collect::<Vec<(Range, Range, Range)>>()
            })
            .map(|(x_range, y_range, z_range)| Cuboid {
                is_on: self.is_on,
                x_range,
                y_range,
                z_range,
            })
            .filter(|cuboid| !cuboid.fits_within(other))
            .collect()
    }
}

impl fmt::Debug for Cuboid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} x={:?},y={:?},z={:?}",
            if self.is_on { "on" } else { "off" },
            self.x_range,
            self.y_range,
            self.z_range,
        )
    }
}

impl FromStr for Cuboid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = if let Some(v) = find_indices(s) {
            v
        } else {
            return Err(parse_error!("Invalid input string: {}", s));
        };

        Ok(Cuboid {
            is_on: s.starts_with("on"),
            x_range: s[i[0]..i[1]].parse()?,
            y_range: s[i[2]..i[3]].parse()?,
            z_range: s[i[4]..].parse()?,
        })
    }
}

fn find_indices(s: &str) -> Option<[usize; 5]> {
    let mut i = [0; 5];

    // x_range.from
    i[0] = 2 + s.find("x=")?;

    // x_range.to
    i[1] = i[0] + s[i[0]..].find(",y=")?;

    // y_range.from
    i[2] = i[1] + 3;

    // y_range.to
    i[3] = i[2] + s[i[2]..].find(",z=")?;

    // z_range.from
    i[4] = i[3] + 3;

    Some(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fits_within() {
        let cuboid_a = Cuboid {
            is_on: true,
            x_range: Range::new(-5, 5),
            y_range: Range::new(-5, 5),
            z_range: Range::new(-5, 5),
        };
        let cuboid_b = Cuboid {
            is_on: true,
            x_range: Range::new(-10, 10),
            y_range: Range::new(-10, 10),
            z_range: Range::new(-10, 10),
        };

        assert!(cuboid_a.fits_within(&cuboid_a));
        assert!(cuboid_a.fits_within(&cuboid_b));
        assert!(cuboid_b.fits_within(&cuboid_b));
        assert!(!cuboid_b.fits_within(&cuboid_a));
    }

    #[test]
    fn test_size() {
        let cuboid = Cuboid {
            is_on: true,
            x_range: Range::new(-5, 5),
            y_range: Range::new(-5, 5),
            z_range: Range::new(-5, 5),
        };
        assert_eq!(cuboid.size(), 11 * 11 * 11);
    }

    #[test]
    fn test_from_str_1() {
        let cuboid = "on x=1..2,y=3..4,z=5..6".parse::<Cuboid>().unwrap();

        assert!(cuboid.is_on);
        assert_eq!(cuboid.x_range.from, 1);
        assert_eq!(cuboid.x_range.to, 2);
        assert_eq!(cuboid.y_range.from, 3);
        assert_eq!(cuboid.y_range.to, 4);
        assert_eq!(cuboid.z_range.from, 5);
        assert_eq!(cuboid.z_range.to, 6);
    }

    #[test]
    fn test_from_str_2() {
        let cuboid = "off x=1..2,y=3..4,z=5..6".parse::<Cuboid>().unwrap();

        assert!(!cuboid.is_on);
        assert_eq!(cuboid.x_range.from, 1);
        assert_eq!(cuboid.x_range.to, 2);
        assert_eq!(cuboid.y_range.from, 3);
        assert_eq!(cuboid.y_range.to, 4);
        assert_eq!(cuboid.z_range.from, 5);
        assert_eq!(cuboid.z_range.to, 6);
    }

    #[test]
    fn test_from_str_3() {
        let result = "invalid".parse::<Cuboid>();
        assert!(result.is_err());
    }
}
