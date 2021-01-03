use std::mem;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::MultilineFromStr;

#[derive(Debug)]
pub struct Area {
    pub coordinates: Vec<(i32, i32)>,
    top: i32,
    right: i32,
    bottom: i32,
    left: i32,
}

impl Area {
    #[cfg(test)]
    pub fn new(coordinates: &[(i32, i32)]) -> Area {
        let mut area = Area {
            coordinates: coordinates.to_vec(),
            top: i32::MIN,
            right: i32::MIN,
            bottom: i32::MAX,
            left: i32::MAX,
        };

        for &(x, y) in coordinates {
            if y > area.top {
                area.top = y;
            }
            if x > area.right {
                area.right = x;
            }
            if y < area.bottom {
                area.bottom = y;
            }
            if x < area.left {
                area.left = x;
            }
        }

        area
    }

    pub fn distance(&self, p1: (i32, i32), p2: (i32, i32)) -> i32 {
        (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
    }

    pub fn distance_to_all(&self, point: (i32, i32)) -> i32 {
        self.coordinates
            .iter()
            .map(|c| self.distance(*c, point))
            .sum()
    }

    pub fn get_closest(&self, p1: (i32, i32)) -> Option<(i32, i32)> {
        let distances: Vec<((i32, i32), i32)> = self
            .coordinates
            .iter()
            .map(|p2| (*p2, self.distance(p1, *p2)))
            .collect();

        let shortest_distance = distances.iter().map(|(_, d)| *d).min()?;

        let closest_coordinates: Vec<(i32, i32)> = distances
            .iter()
            .filter(|(_, d)| *d == shortest_distance)
            .map(|(p, _)| *p)
            .collect();

        if closest_coordinates.len() == 1 {
            Some(closest_coordinates[0])
        } else {
            None
        }
    }

    pub fn edge(&self) -> Vec<(i32, i32)> {
        let mut edge = Vec::new();
        for x in self.left - 1..self.right + 1 {
            edge.push((x + 1, self.bottom - 1));
            edge.push((x, self.top + 1));
        }
        for y in self.bottom - 1..self.top + 1 {
            edge.push((self.left - 1, y + 1));
            edge.push((self.right + 1, y));
        }
        edge
    }

    pub fn inner(&self) -> Vec<(i32, i32)> {
        let mut inner = Vec::new();
        for x in self.left..=self.right {
            for y in self.bottom..=self.top {
                inner.push((x, y));
            }
        }
        inner
    }

    pub fn region(&self, max_distance: i32) -> Vec<(i32, i32)> {
        let distance = max_distance / (self.coordinates.len() as i32);
        let mut x_min = self.left - distance;
        let mut x_max = self.right + distance;
        if x_min > x_max {
            mem::swap(&mut x_min, &mut x_max);
        }
        let mut y_min = self.bottom - distance;
        let mut y_max = self.top + distance;
        if y_min > y_max {
            mem::swap(&mut y_min, &mut y_max);
        }

        let mut region = Vec::new();
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                region.push((x, y));
            }
        }
        region
    }
}

impl MultilineFromStr for Area {
    type Err = ParseError;

    fn new() -> Self {
        Area {
            coordinates: Vec::new(),
            top: i32::MIN,
            right: i32::MIN,
            bottom: i32::MAX,
            left: i32::MAX,
        }
    }

    fn indicates_new_record(&self, _line: &str) -> bool {
        false
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        let parts: Vec<&str> = line.split(", ").collect();
        let x = parts[0].parse()?;
        let y = parts[1].parse()?;
        self.coordinates.push((x, y));

        if y > self.top {
            self.top = y;
        }
        if x > self.right {
            self.right = x;
        }
        if y < self.bottom {
            self.bottom = y;
        }
        if x < self.left {
            self.left = x;
        }

        Ok(())
    }
}
