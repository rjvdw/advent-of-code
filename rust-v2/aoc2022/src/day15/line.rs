use std::fmt;

use crate::point::Point;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Line(pub i64, pub i64);

impl Line {
    pub fn find_intersection_point(
        &self,
        other: &Line,
        lower_bound: i64,
        upper_bound: i64,
    ) -> Option<Point> {
        if self.0 == other.0 {
            // lines are parallel
            None
        } else {
            let a = self.0 - other.0;
            let b = other.1 - self.1;

            if b % a == 0 {
                let x = b / a;
                let y = self.eval(x);

                if x >= lower_bound && x <= upper_bound && y >= lower_bound && y <= upper_bound {
                    Some(Point(x, y))
                } else {
                    None
                }
            } else {
                // no integer solution
                None
            }
        }
    }

    pub fn eval(&self, x: i64) -> i64 {
        self.0 * x + self.1
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "y = {}x + {}", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let line = Line(3, 1);
        assert_eq!(line.eval(0), 1);
        assert_eq!(line.eval(1), 4);
        assert_eq!(line.eval(2), 7);
    }

    #[test]
    fn test_find_intersection_point() {
        let line1 = Line(3, 1);
        let line2 = Line(3, 2);
        let line3 = Line(4, 1);
        let line4 = Line(-3, 4);

        assert_eq!(line1.find_intersection_point(&line1, -100, 100), None);
        assert_eq!(line1.find_intersection_point(&line2, -100, 100), None);
        assert_eq!(
            line1.find_intersection_point(&line3, -100, 100),
            Some(Point(0, 1))
        );
        assert_eq!(
            line2.find_intersection_point(&line3, -100, 100),
            Some(Point(1, 5))
        );
        assert_eq!(line1.find_intersection_point(&line4, -100, 100), None);
    }
}
