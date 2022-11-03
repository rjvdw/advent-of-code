use std::cmp::Ordering;
use std::io;

use itertools::Itertools;
use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TargetArea {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl TargetArea {
    /// To find the maximum height, we just need to focus on the vertical velocity. The horizontal
    /// velocity does not impact the height of the trajectory. In principle we should verify that
    /// the trajectory does go through the target area, considering the horizontal component, but we
    /// can probably ignore this for this puzzle.
    ///
    /// For the vertical velocity, we know that once the probe reaches its maximum height, its
    /// vertical velocity will be 0. So we don't actually care about its initial velocity, so long
    /// as the maximum height can actually be reached from the starting position, and the probe
    /// falls through the target area on its way down.
    ///
    /// When falling down, the probe will follow the n(n+1)/2 formula (every step the height
    /// decreases with one more than the previous step). Since the probe has to pass through 0 and
    /// the target area, we should find the point in the target area that is furthest from 0. The
    /// distance between this point and 0 gives us n.
    pub fn find_max_height(&self) -> i64 {
        let (d, y) = self.get_y_bound();
        match y.cmp(&0) {
            Ordering::Less => d * (d - 1) / 2,
            Ordering::Equal => panic!("The target area contains a point with y=0. This means the trajectory can be arbitrarily high."),
            Ordering::Greater => d * (d + 1) / 2,
        }
    }

    /// In order to find all valid initial velocities, we need to have some bounds on the search
    /// area. For the initial horizontal velocity, this is easy: The initial velocity must be low
    /// enough to not overshoot the target area in the first step. For the vertical velocity we can
    /// use the information from the previous part (the velocity that reaches the max height is also
    /// the max velocity).
    ///
    /// TODO: Optimize. (E.g. pick a better lower bound for v_x.)
    pub fn find_all_valid_initial_velocities(&self) -> Vec<(i64, i64)> {
        let (d, _) = self.get_y_bound();
        (0..=self.x_max)
            .cartesian_product(-d..d)
            .filter(|&c| self.is_valid_trajectory(c))
            .collect()
    }

    fn get_y_bound(&self) -> (i64, i64) {
        if self.y_min < 0 && self.y_max < 0 {
            (self.y_min.abs(), self.y_min)
        } else if self.y_min > 0 && self.y_max > 0 {
            (self.y_max.abs(), self.y_max)
        } else {
            (0, 0)
        }
    }

    fn is_valid_trajectory(&self, v_initial: (i64, i64)) -> bool {
        let (mut v_x, mut v_y) = v_initial;
        let mut x = 0;
        let mut y = 0;

        loop {
            x += v_x;
            y += v_y;

            if x > self.x_max {
                // we have overshot the target area
                return false;
            }

            if v_y < 0 && y < self.y_min {
                // we have overshot the target area
                return false;
            }

            if x >= self.x_min && y >= self.y_min && y <= self.y_max {
                return true;
            }

            if v_x == 0 && x < self.x_min {
                // x will no longer change, so we will never reach the target area
                return false;
            }

            if v_x != 0 {
                v_x -= v_x / v_x.abs();
            }
            v_y -= 1;
        }
    }

    pub fn parse<I>(mut lines: I) -> Result<TargetArea, ParseError>
    where
        I: Iterator<Item = io::Result<String>>,
    {
        let line = lines.next().unwrap()?;
        let s = line.strip_prefix("target area: x=").unwrap();
        let p1 = s.find("..").unwrap();
        let x_min = s[0..p1].parse()?;
        let p2 = s.find(", y=").unwrap();
        let x_max = s[p1 + 2..p2].parse()?;
        let p3 = p2 + 4;
        let p4 = p3 + s[p3..].find("..").unwrap();
        let y_min = s[p3..p4].parse()?;
        let y_max = s[p4 + 2..].parse()?;

        Ok(TargetArea {
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_initial_velocity_with_max_height() {
        assert_eq!(target_area().find_max_height(), 45);
    }

    #[test]
    fn test_find_all_valid_initial_velocities() {
        assert_eq!(target_area().find_all_valid_initial_velocities().len(), 112);
    }

    #[test]
    fn test_parse_input() {
        let input = vec![Ok("target area: x=20..30, y=-10..-5".to_string())];
        assert_eq!(TargetArea::parse(input.into_iter()), Ok(target_area()));
    }

    fn target_area() -> TargetArea {
        TargetArea {
            x_min: 20,
            x_max: 30,
            y_min: -10,
            y_max: -5,
        }
    }
}
