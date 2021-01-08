use rdcl_aoc_helpers::search::Navigable;

pub struct Maze(pub u64);

impl Maze {
    pub fn is_wall(&self, (x, y): (u64, u64)) -> bool {
        let mut code = x * x + 3 * x + 2 * x * y + y + y * y + self.0;
        let mut is_wall = false;
        while code > 0 {
            if code % 2 == 1 {
                is_wall = !is_wall;
            }
            code /= 2;
        }
        is_wall
    }
}

impl Navigable for Maze {
    type Point = (u64, u64);

    fn distance_score(&self, a: &Self::Point, b: &Self::Point) -> u64 {
        (a.0.max(b.0) - a.0.min(b.0)) + (a.1.max(b.1) - a.1.min(b.1))
    }

    fn get_neighbours(&self, &(x, y): &Self::Point) -> Vec<(u64, Self::Point)> {
        let mut n = Vec::new();
        if x > 0 {
            n.push((x - 1, y));
        }
        if y > 0 {
            n.push((x, y - 1));
        }
        n.push((x + 1, y));
        n.push((x, y + 1));
        n.iter()
            .filter(|&c| !self.is_wall(*c))
            .map(|&c| (1, c))
            .collect()
    }
}
