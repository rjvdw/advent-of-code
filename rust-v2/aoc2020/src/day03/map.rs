use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Map {
    grid: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn parse<T>(input: T) -> Map
    where
        T: Iterator<Item = String>,
    {
        let mut map = Map::default();
        for (row, line) in input.enumerate() {
            map.height = row + 1;
            for (col, ch) in line.chars().enumerate() {
                map.width = col + 1;
                if ch == '#' {
                    map.grid.insert((row, col));
                }
            }
        }
        map
    }

    pub fn has_tree(&self, row: usize, col: usize) -> bool {
        self.grid.contains(&(row, col))
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
}
