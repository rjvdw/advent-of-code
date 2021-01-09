use crate::nanobot::Nanobot;
use crate::Point;

const SMALL_THRESHOLD: i64 = 10;

/// A region within space, bounded by 2 coordinates.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Region {
    pub from: Point,
    pub to: Point,
}

impl Region {
    /// Splits a region into at most 8 smaller regions. If the region is sufficiently big, it will
    /// be split into 8 regions. If the region is too small in one or more axes, the region will be
    /// split into 4 regions (if the region is too small in just one axis), 2 regions (if the region
    /// is too small in two axes), or just 1 region (if the region is too small in all axes). In
    /// this last case, the `is_small_enough` method would also return true, and this method should
    /// not be called.
    pub fn split(&self) -> Vec<Region> {
        let x_range = self.get_range_for_split(self.from.0, self.to.0);
        let y_range = self.get_range_for_split(self.from.1, self.to.1);
        let z_range = self.get_range_for_split(self.from.2, self.to.2);

        let mut sub_regions = Vec::new();
        for &x in &x_range {
            for &y in &y_range {
                for &z in &z_range {
                    sub_regions.push(Region {
                        from: (x.0, y.0, z.0),
                        to: (x.1, y.1, z.1),
                    });
                }
            }
        }
        sub_regions
    }

    /// Determines in how many pieces the region should be split in a specific axis (either 1 or 2).
    fn get_range_for_split(&self, from: i64, to: i64) -> Vec<(i64, i64)> {
        if to - from < SMALL_THRESHOLD {
            vec![(from, to)]
        } else {
            let middle = (from + to) / 2;
            vec![(from, middle), (middle + 1, to)]
        }
    }

    /// Count the number of nanobots whose sphere of influence overlap with this region.
    pub fn count_influencing_nanobots(&self, nanobots: &[Nanobot]) -> usize {
        nanobots.iter().filter(|&b| b.influences(self)).count()
    }

    /// Returns true when a region is small enough so that we can manually loop over all points.
    pub fn is_small_enough(&self) -> bool {
        self.to.0 - self.from.0 < SMALL_THRESHOLD
            && self.to.1 - self.from.1 < SMALL_THRESHOLD
            && self.to.2 - self.from.2 < SMALL_THRESHOLD
    }

    /// Returns an iterator over all points within this region.
    pub fn iter(&self) -> RegionIter {
        RegionIter::new(self)
    }

    /// Checks whether a point is contained within this region.
    pub fn contains(&self, point: Point) -> bool {
        (self.from.0..=self.to.0).contains(&point.0)
            && (self.from.1..=self.to.1).contains(&point.1)
            && (self.from.2..=self.to.2).contains(&point.2)
    }

    /// Returns the corners of this region.
    pub fn get_corners(&self) -> Vec<Point> {
        vec![
            (self.from.0, self.from.1, self.from.2),
            (self.to.0, self.from.1, self.from.2),
            (self.from.0, self.to.1, self.from.2),
            (self.from.0, self.from.1, self.to.2),
            (self.to.0, self.to.1, self.from.2),
            (self.to.0, self.from.1, self.to.2),
            (self.from.0, self.to.1, self.to.2),
            (self.to.0, self.to.1, self.to.2),
        ]
    }
}

/// Iterator over all points within a region.
pub struct RegionIter {
    from: Point,
    to: Point,
    current: Point,
}

impl RegionIter {
    /// Constructs a fresh iterator from a region.
    fn new(region: &Region) -> RegionIter {
        RegionIter {
            from: region.from,
            to: region.to,
            current: (region.from.0 - 1, region.from.1, region.from.2),
        }
    }
}

impl Iterator for RegionIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.0 += 1;
        if self.current.0 > self.to.0 {
            self.current.0 = self.from.0;
            self.current.1 += 1;
        }
        if self.current.1 > self.to.1 {
            self.current.1 = self.from.1;
            self.current.2 += 1;
        }
        if self.current.2 > self.to.2 {
            None
        } else {
            Some(self.current)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_large_region_into_8_sub_regions() {
        let region = Region {
            from: (-50, -50, -50),
            to: (50, 50, 50),
        };
        let sub_regions = region.split();
        assert_eq!(
            sub_regions.len(),
            8,
            "The number of sub regions is incorrect."
        );
        assert!(
            region
                .get_corners()
                .iter()
                .all(|&p| sub_regions.iter().any(|r| r.contains(p))),
            "The corners of the region are not contained within the sub regions."
        );
        assert_eq!(
            region.iter().count(),
            sub_regions.iter().map(|r| r.iter().count()).sum::<usize>(),
            "The total number of points in the sub regions does not match the number of points within the region."
        );
    }

    #[test]
    fn test_split_region_that_is_small_in_one_axis_into_4_sub_regions() {
        let region = Region {
            from: (-2, -50, -50),
            to: (2, 50, 50),
        };
        let sub_regions = region.split();
        assert_eq!(
            sub_regions.len(),
            4,
            "The number of sub regions is incorrect."
        );
        assert!(
            region
                .get_corners()
                .iter()
                .all(|&p| sub_regions.iter().any(|r| r.contains(p))),
            "The corners of the region are not contained within the sub regions."
        );
        assert!(region
            .get_corners()
            .iter()
            .all(|&p| sub_regions.iter().any(|r| r.contains(p))));
    }

    #[test]
    fn test_split_region_that_is_small_in_two_axes_into_2_sub_regions() {
        let region = Region {
            from: (-2, -2, -50),
            to: (2, 2, 50),
        };
        let sub_regions = region.split();
        assert_eq!(
            sub_regions.len(),
            2,
            "The number of sub regions is incorrect."
        );
        assert!(
            region
                .get_corners()
                .iter()
                .all(|&p| sub_regions.iter().any(|r| r.contains(p))),
            "The corners of the region are not contained within the sub regions."
        );
        assert!(region
            .get_corners()
            .iter()
            .all(|&p| sub_regions.iter().any(|r| r.contains(p))));
    }

    #[test]
    fn test_split_small_region_into_1_sub_region() {
        let region = Region {
            from: (-2, -2, -2),
            to: (2, 2, 2),
        };
        let sub_regions = region.split();
        assert_eq!(
            sub_regions.len(),
            1,
            "The number of sub regions is incorrect."
        );
        assert_eq!(sub_regions[0], region);
    }

    #[test]
    fn test_iter() {
        let region = Region {
            from: (-1, -1, -1),
            to: (1, 1, 1),
        };
        let mut iter = region.iter();

        assert_eq!(iter.next(), Some((-1, -1, -1)));
        assert_eq!(iter.next(), Some((0, -1, -1)));
        assert_eq!(iter.next(), Some((1, -1, -1)));

        assert_eq!(iter.next(), Some((-1, 0, -1)));
        assert_eq!(iter.next(), Some((0, 0, -1)));
        assert_eq!(iter.next(), Some((1, 0, -1)));

        assert_eq!(iter.next(), Some((-1, 1, -1)));
        assert_eq!(iter.next(), Some((0, 1, -1)));
        assert_eq!(iter.next(), Some((1, 1, -1)));

        assert_eq!(iter.next(), Some((-1, -1, 0)));
        assert_eq!(iter.next(), Some((0, -1, 0)));
        assert_eq!(iter.next(), Some((1, -1, 0)));

        assert_eq!(iter.next(), Some((-1, 0, 0)));
        assert_eq!(iter.next(), Some((0, 0, 0)));
        assert_eq!(iter.next(), Some((1, 0, 0)));

        assert_eq!(iter.next(), Some((-1, 1, 0)));
        assert_eq!(iter.next(), Some((0, 1, 0)));
        assert_eq!(iter.next(), Some((1, 1, 0)));

        assert_eq!(iter.next(), Some((-1, -1, 1)));
        assert_eq!(iter.next(), Some((0, -1, 1)));
        assert_eq!(iter.next(), Some((1, -1, 1)));

        assert_eq!(iter.next(), Some((-1, 0, 1)));
        assert_eq!(iter.next(), Some((0, 0, 1)));
        assert_eq!(iter.next(), Some((1, 0, 1)));

        assert_eq!(iter.next(), Some((-1, 1, 1)));
        assert_eq!(iter.next(), Some((0, 1, 1)));
        assert_eq!(iter.next(), Some((1, 1, 1)));

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_corners_are_contained_in_region() {
        let region = Region {
            from: (-3, -3, -3),
            to: (3, 3, 3),
        };
        let corners = region.get_corners();

        assert_eq!(corners.len(), 8);

        assert_eq!(corners[0], (-3, -3, -3));
        assert_eq!(corners[1], (3, -3, -3));
        assert_eq!(corners[2], (-3, 3, -3));
        assert_eq!(corners[3], (-3, -3, 3));
        assert_eq!(corners[4], (3, 3, -3));
        assert_eq!(corners[5], (3, -3, 3));
        assert_eq!(corners[6], (-3, 3, 3));
        assert_eq!(corners[7], (3, 3, 3));

        assert!(corners.iter().all(|&c| region.contains(c)));
    }
}
