use std::str::FromStr;
use std::{fmt, mem};

use rdcl_aoc_algebra::algebra3d::Vector3d;
use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::{err_parse_error, ParseResult};

use crate::fraction::Fraction;

#[derive(Debug, Copy, Clone, Default)]
pub struct Trajectory {
    position: Vector3d<i64>,
    velocity: Vector3d<i64>,
}

impl Trajectory {
    /// Consider only the X and Y axes; ignore the Z axis. Looking
    /// forward in time, how many of the hailstones' paths will
    /// intersect within a test area? (The hailstones themselves don't
    /// have to collide, just test for intersections between the paths
    /// they will trace.)
    ///
    /// To solve this, we need to find times t1 and t2 such that
    /// hailstone `self` is at the same position at t1 as hailstone
    /// `other` at t2. This basically equates to solving a system of two
    /// linear equations with two unknowns (t1 and t2):
    ///
    ///     self.velocity.0 * t1 + self.position.0 == other.velocity.0 * t2 + other.position.0
    ///     self.velocity.1 * t1 + self.position.1 == other.velocity.1 * t2 + other.position.1
    pub fn estimate_collision_between(&self, other: Trajectory, bounds: (i64, i64)) -> bool {
        if let Some((t1, t2)) = self.find_possible_collision(other) {
            let v1 = self.eval_at(t1);
            let v2 = other.eval_at(t2);

            t1 > 0
                && t2 > 0
                && v1.0.is_between(bounds.0, bounds.1)
                && v1.1.is_between(bounds.0, bounds.1)
                && v2.0.is_between(bounds.0, bounds.1)
                && v2.1.is_between(bounds.0, bounds.1)
        } else {
            false
        }
    }

    fn find_possible_collision(&self, other: Trajectory) -> Option<(Fraction, Fraction)> {
        let mut r1: Vector3d<Fraction> = to_solver_row(*self, other, |p| p.0);
        let mut r2: Vector3d<Fraction> = to_solver_row(*self, other, |p| p.1);

        // check if solvable at all
        if r1.0 == 0 {
            if r2.0 == 0 {
                todo!()
            }
            mem::swap(&mut r1, &mut r2);
        }

        if r1.1 == 0 && r2.1 == 0 {
            todo!()
        }

        // solve
        r1 /= r1.0;
        r2 -= r1 * r2.0;

        if r2.1 == 0 {
            // paths are parallel and will never cross
            return None;
        }

        r2 /= r2.1;
        r1 -= r2 * r1.1;

        Some((r1.2, r2.2))
    }

    fn eval_at(&self, t: Fraction) -> Vector3d<Fraction> {
        Vector3d(
            t * self.velocity.0 + self.position.0,
            t * self.velocity.1 + self.position.1,
            t * self.velocity.2 + self.position.2,
        )
    }
}

fn to_solver_row<T>(one: Trajectory, other: Trajectory, by_key: T) -> Vector3d<Fraction>
where
    T: Fn(Vector3d<i64>) -> i64,
{
    Vector3d(
        Fraction::new(by_key(one.velocity), 1),
        Fraction::new(-by_key(other.velocity), 1),
        Fraction::new(by_key(other.position) - by_key(one.position), 1),
    )
}

impl std::ops::Sub<Trajectory> for Trajectory {
    type Output = Trajectory;

    fn sub(self, rhs: Trajectory) -> Self::Output {
        Trajectory {
            position: self.position - rhs.position,
            velocity: self.velocity - rhs.velocity,
        }
    }
}

impl FromStr for Trajectory {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hailstone = Trajectory::default();
        let s = s.replace(' ', "");
        let mut parts = s.split('@').map(parse_vector);
        hailstone.position = parts.next().ok_or(())??;
        hailstone.velocity = parts.next().ok_or(())??;
        if parts.next().is_none() {
            Ok(hailstone)
        } else {
            err_parse_error!("invalid hail stone: {}", s)
        }
    }
}

fn parse_vector(s: &str) -> ParseResult<Vector3d<i64>> {
    let mut v = Vector3d::default();
    let mut parts = s.split(',').map(|p| p.parse::<i64>());
    v.0 = parts.next().ok_or(())??;
    v.1 = parts.next().ok_or(())??;
    v.2 = parts.next().ok_or(())??;
    if parts.next().is_none() {
        Ok(v)
    } else {
        err_parse_error!("invalid vec3d: {}", s)
    }
}

impl fmt::Display for Trajectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {} @ {}, {}, {}",
            self.position.0,
            self.position.1,
            self.position.2,
            self.velocity.0,
            self.velocity.1,
            self.velocity.2,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimation() {
        let hs1 = "19, 13, 30 @ -2, 1, -2".parse::<Trajectory>().unwrap();
        let hs2 = "18, 19, 22 @ -1, -1, -2".parse::<Trajectory>().unwrap();
        assert!(hs1.estimate_collision_between(hs2, (7, 27)));
    }
}
