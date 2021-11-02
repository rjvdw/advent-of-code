use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Moon {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
}

/// Updates the velocity vector of a moon in the given axis.
macro_rules! update_velocity {
    ($updated:expr, $self:expr, $other:expr, $x:tt) => {
        $updated.velocity.$x += match $self.position.$x.cmp(&$other.position.$x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
    };
}

impl Moon {
    pub fn new(x: i64, y: i64, z: i64) -> Moon {
        Moon {
            position: (x, y, z),
            velocity: (0, 0, 0),
        }
    }

    pub fn get_position(&self) -> (i64, i64, i64) {
        self.position
    }

    pub fn get_velocity(&self) -> (i64, i64, i64) {
        self.velocity
    }

    pub fn update_moon(&self, moons: &[Moon]) -> Moon {
        let mut updated = *self;
        for moon in moons {
            update_velocity![updated, self, moon, 0];
            update_velocity![updated, self, moon, 1];
            update_velocity![updated, self, moon, 2];
        }
        updated.position.0 += updated.velocity.0;
        updated.position.1 += updated.velocity.1;
        updated.position.2 += updated.velocity.2;
        updated
    }

    pub fn potential_energy(&self) -> i64 {
        self.position.0.abs() + self.position.1.abs() + self.position.2.abs()
    }

    pub fn kinetic_energy(&self) -> i64 {
        self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs()
    }

    pub fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }
}

impl Display for Moon {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<x={}, y={}, z={}>",
            self.position.0, self.position.1, self.position.2
        )
    }
}

impl FromStr for Moon {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Moon::new(
            parse_position(s, 'x')?,
            parse_position(s, 'y')?,
            parse_position(s, 'z')?,
        ))
    }
}

fn parse_position(s: &str, ch: char) -> Result<i64, ParseError> {
    let left = 1 + match s.find(ch) {
        Some(idx) => idx + 1,
        None => {
            return Err(parse_error!(
                "Invalid {} coordinate encountered in '{}'.",
                ch,
                s
            ));
        }
    };
    let right = left
        + match s[left..].find(',') {
            Some(idx) => idx,
            None => match s[left..].find('>') {
                Some(idx) => idx,
                None => {
                    return Err(parse_error!(
                        "Invalid {} coordinate encountered in '{}'.",
                        ch,
                        s
                    ));
                }
            },
        };
    match s[left..right].parse() {
        Ok(v) => Ok(v),
        Err(e) => Err(parse_error!(
            "Invalid {} coordinate ({}) encountered in '{}': {}",
            ch,
            &s[left..right],
            s,
            e
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_moon() {
        let moons = vec![
            moon((-1, 0, 2), (0, 0, 0)),
            moon((2, -10, -7), (0, 0, 0)),
            moon((4, -8, 8), (0, 0, 0)),
            moon((3, 5, -1), (0, 0, 0)),
        ];

        let moons: Vec<Moon> = moons.iter().map(|moon| moon.update_moon(&moons)).collect();
        assert_eq!(
            moons,
            vec![
                moon((2, -1, 1), (3, -1, -1)),
                moon((3, -7, -4), (1, 3, 3)),
                moon((1, -7, 5), (-3, 1, -3)),
                moon((2, 2, 0), (-1, -3, 1)),
            ]
        );

        let moons: Vec<Moon> = moons.iter().map(|moon| moon.update_moon(&moons)).collect();
        assert_eq!(
            moons,
            vec![
                moon((5, -3, -1), (3, -2, -2)),
                moon((1, -2, 2), (-2, 5, 6)),
                moon((1, -4, -1), (0, 3, -6)),
                moon((1, -4, 2), (-1, -6, 2)),
            ]
        );
    }

    #[test]
    fn test_energy_1() {
        let moon = moon((2, 1, -3), (-3, -2, 1));
        assert_eq!(moon.potential_energy(), 6);
        assert_eq!(moon.kinetic_energy(), 6);
        assert_eq!(moon.total_energy(), 36);
    }

    #[test]
    fn test_energy_2() {
        let moon = moon((1, -8, 0), (-1, 1, 3));
        assert_eq!(moon.potential_energy(), 9);
        assert_eq!(moon.kinetic_energy(), 5);
        assert_eq!(moon.total_energy(), 45);
    }

    fn moon(position: (i64, i64, i64), velocity: (i64, i64, i64)) -> Moon {
        Moon { position, velocity }
    }
}
