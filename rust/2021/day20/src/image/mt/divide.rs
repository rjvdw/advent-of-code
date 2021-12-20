use crate::bounds::Bounds;
use crate::point::Point;

pub(crate) const MAX_REGION_SIZE: usize = 2048; // this seems to be a happy medium

pub(crate) trait Dividable {
    fn divide(&self) -> Vec<Self>
    where
        Self: Sized;
}

impl Dividable for Bounds {
    /// Divides the area specified by `Bounds` into smaller regions.
    fn divide(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        let mut regions = vec![];
        let mut stack = vec![*self];

        while let Some(region) = stack.pop() {
            let top = region.top_left.row;
            let left = region.top_left.col;
            let bottom = region.bottom_right.row;
            let right = region.bottom_right.col;

            let height = region.height();
            let width = region.width();

            let (r1, r2): (Bounds, Bounds) = if height > width {
                let mid_point = top + (height / 2) as i64;
                (
                    Bounds {
                        top_left: Point::new(top, left),
                        bottom_right: Point::new(mid_point, right),
                    },
                    Bounds {
                        top_left: Point::new(mid_point + 1, left),
                        bottom_right: Point::new(bottom, right),
                    },
                )
            } else {
                let mid_point = left + (width / 2) as i64;
                (
                    Bounds {
                        top_left: Point::new(top, left),
                        bottom_right: Point::new(bottom, mid_point),
                    },
                    Bounds {
                        top_left: Point::new(top, mid_point + 1),
                        bottom_right: Point::new(bottom, right),
                    },
                )
            };

            if r1.size() <= MAX_REGION_SIZE {
                regions.push(r1);
            } else {
                stack.push(r1);
            }

            if r2.size() <= MAX_REGION_SIZE {
                regions.push(r2);
            } else {
                stack.push(r2);
            }
        }

        regions
    }
}
