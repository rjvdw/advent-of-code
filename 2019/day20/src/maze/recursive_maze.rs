use std::collections::{HashMap, HashSet, VecDeque};

use grid::Grid;

use crate::tile::Tile;

pub struct RecursiveMaze {
    pub(crate) layout: Grid<Tile>,
    pub(crate) portals: HashMap<(usize, usize), (usize, usize)>,
    pub(crate) start: (usize, usize, usize),
    pub(crate) end: (usize, usize, usize),
}

impl RecursiveMaze {
    pub fn find_shortest_route(&self) -> Option<usize> {
        let mut exploring = VecDeque::new();
        exploring.push_back((self.start, 0));
        let mut seen = HashSet::new();
        seen.insert(self.start);

        while let Some((p, distance)) = exploring.pop_front() {
            // println!("{} {:?}", distance, p);
            for neighbour in self.get_neighbours(p) {
                if neighbour == self.end {
                    return Some(distance + 1);
                }
                if !seen.contains(&neighbour) {
                    seen.insert(neighbour);
                    exploring.push_back((neighbour, distance + 1));
                }
            }
        }

        None
    }

    fn get_neighbours(&self, (x, y, z): (usize, usize, usize)) -> Vec<(usize, usize, usize)> {
        // assumption: the edges of the map are not used, so we do not need to do bound checks
        let mut neighbours = vec![(x, y - 1, z), (x, y + 1, z), (x - 1, y, z), (x + 1, y, z)];

        if let Some(&(x1, y1)) = self.portals.get(&(x, y)) {
            let threshold_x = self.layout.cols() / 4;
            let threshold_y = self.layout.rows() / 4;
            let z1 =
                if x < threshold_x || x > 3 * threshold_x || y < threshold_y || y > 3 * threshold_y
                {
                    // outer ring
                    z - 1
                } else {
                    // inner ring
                    z + 1
                };

            neighbours.push((x1, y1, z1));
        }

        neighbours
            .iter()
            .filter(|&&(x, y, z)| z > 0 && self.layout[y][x].is_open())
            .copied()
            .collect()
    }
}
