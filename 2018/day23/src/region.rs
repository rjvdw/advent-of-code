use crate::nanobot::Nanobot;
use crate::{distance_to_origin, Point, ORIGIN};

const SMALL_THRESHOLD: i64 = 10;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Region {
    pub from: Point,
    pub to: Point,
}

impl Region {
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

    fn get_range_for_split(&self, from: i64, to: i64) -> Vec<(i64, i64)> {
        if to - from < SMALL_THRESHOLD {
            vec![(from, to)]
        } else {
            let middle = (from + to) / 2;
            vec![(from, middle), (middle + 1, to)]
        }
    }

    pub fn count_influencing_nanobots(&self, nanobots: &[Nanobot]) -> usize {
        nanobots.iter().filter(|&b| b.influences(self)).count()
    }

    /// Returns true if, for all nanobots that can influence this region, the region is fully
    /// within their sphere of influence.
    pub fn is_fully_contained(&self, nanobots: &[Nanobot]) -> bool {
        nanobots
            .iter()
            .filter(|&b| b.influences(self))
            .all(|&b| b.all_in_range(self))
    }

    pub fn is_small_enough(&self) -> bool {
        self.to.0 - self.from.0 < SMALL_THRESHOLD
            && self.to.1 - self.from.1 < SMALL_THRESHOLD
            && self.to.2 - self.from.2 < SMALL_THRESHOLD
    }

    pub fn iter(&self) -> RegionIter {
        RegionIter::new(self)
    }

    pub fn contains(&self, point: Point) -> bool {
        (self.from.0..=self.to.0).contains(&point.0)
            && (self.from.1..=self.to.1).contains(&point.1)
            && (self.from.2..=self.to.2).contains(&point.2)
    }

    pub fn get_corners(&self) -> Vec<Point> {
        let (x0, y0, z0) = self.from;
        let (x1, y1, z1) = self.to;

        vec![
            (x0, y0, z0),
            (x1, y0, z0),
            (x0, y1, z0),
            (x0, y0, z1),
            (x1, y1, z0),
            (x1, y0, z1),
            (x0, y1, z1),
            (x1, y1, z1),
        ]
    }

    pub fn closest_point_to_origin(&self) -> Point {
        if self.contains(ORIGIN) {
            ORIGIN
        } else {
            self.get_corners()
                .iter()
                .min_by_key(|&&p| distance_to_origin(p))
                .copied()
                .unwrap()
        }
    }
}

pub struct RegionIter {
    from: Point,
    to: Point,
    current: Point,
}

impl RegionIter {
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
