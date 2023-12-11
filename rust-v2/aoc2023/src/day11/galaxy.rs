use std::collections::HashSet;

use rdcl_aoc_core::input::FromInput;
use rdcl_aoc_core::ParseResult;
use rdcl_aoc_pathfinding::taxi_cab_2d;

#[derive(Debug, Clone, Default)]
pub struct GalaxyMap {
    galaxies: Vec<(usize, usize)>,
    empty_rows: HashSet<usize>,
    empty_columns: HashSet<usize>,
}

impl GalaxyMap {
    pub fn sum_shortest_paths(&self, scaling_factor: usize) -> usize {
        let mut sum = 0;
        for i in 0..self.galaxies.len() - 1 {
            for j in i + 1..self.galaxies.len() {
                let one = self.galaxies[i];
                let other = self.galaxies[j];

                sum += taxi_cab_2d(one, other);

                let expansion = self.expansion_between(one, other);
                if scaling_factor == 0 {
                    sum -= expansion;
                } else {
                    sum += expansion * (scaling_factor - 1);
                }
            }
        }
        sum
    }

    fn expansion_between(&self, one: (usize, usize), other: (usize, usize)) -> usize {
        expansion_between_in_1d(one.0, other.0, &self.empty_rows)
            + expansion_between_in_1d(one.1, other.1, &self.empty_columns)
    }
}

fn expansion_between_in_1d(one: usize, other: usize, empty: &HashSet<usize>) -> usize {
    let from = one.min(other);
    let to = one.max(other);

    (from..to).filter(|v| empty.contains(v)).count()
}

impl FromInput for GalaxyMap {
    fn parse<T>(input: T) -> ParseResult<Self>
    where
        T: Iterator<Item = String>,
    {
        let mut map = GalaxyMap::default();

        for (row, line) in input.enumerate() {
            map.empty_rows.insert(row);
            for (column, ch) in line.chars().enumerate() {
                map.empty_columns.insert(column);
                if ch == '#' {
                    map.galaxies.push((row, column));
                }
            }
        }

        for (row, column) in &map.galaxies {
            map.empty_rows.remove(row);
            map.empty_columns.remove(column);
        }

        Ok(map)
    }
}
