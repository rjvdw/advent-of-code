use std::collections::{HashMap, HashSet, VecDeque};

use grid::Grid;

use rdcl_aoc_core::input::FromInput;
use rdcl_aoc_core::ParseResult;

type Point = (usize, usize);

#[derive(Debug, Copy, Clone)]
struct EdgeTarget {
    point: Point,
    distance: usize,
}

#[derive(Debug, Copy, Clone)]
struct ExploringEntry {
    point: Point,
    last_junction: Point,
    came_from: Option<Point>,
    distance_to_last_junction: usize,
}

impl ExploringEntry {
    fn new(point: Point) -> ExploringEntry {
        ExploringEntry {
            point,
            last_junction: point,
            came_from: None,
            distance_to_last_junction: 0,
        }
    }

    fn next(&self, point: Point) -> ExploringEntry {
        ExploringEntry {
            point,
            last_junction: self.last_junction,
            came_from: Some(self.point),
            distance_to_last_junction: self.distance_to_last_junction + 1,
        }
    }

    fn apply_to(&self, edges: &mut TrailGraphEdges) {
        edges
            .entry(self.last_junction)
            .or_default()
            .push(EdgeTarget {
                point: self.point,
                distance: self.distance_to_last_junction,
            });
    }
}

type TrailGraphEdges = HashMap<Point, Vec<EdgeTarget>>;
type Exploring = VecDeque<ExploringEntry>;

#[derive(Debug)]
struct TrailGraph {
    edges: HashMap<Point, Vec<EdgeTarget>>,
    start: Point,
    end: Point,
}

impl TrailGraph {
    /// Convert a trail map to a trail graph, where only the junctions
    /// are shown together with the possible distances between
    /// junctions.
    ///
    /// It is assumed that the trail map does not contain any clearings,
    /// i.e. there are no 2x2 blocks of tiles that contain no forests.
    fn from(trail: &Trail, slopes_are_slippy: bool) -> TrailGraph {
        // all edges that have been found so far
        let mut edges = TrailGraphEdges::new();

        // all points that are currently being explored
        let mut exploring = Exploring::new();
        exploring.push_back(ExploringEntry::new(trail.start));

        // all junctions that have already been encountered and so do not need exploring again
        let mut junctions: HashSet<(Point, Point)> = HashSet::new();

        while let Some(entry) = exploring.pop_back() {
            let mut neighbours = trail.get_neighbours(entry.point, slopes_are_slippy);
            if let Some(came_from) = entry.came_from {
                neighbours = neighbours
                    .iter()
                    .copied()
                    .filter(|&n| n != came_from)
                    .collect();
            }

            if entry.point == trail.end || neighbours.len() > 1 {
                // this is a junction
                if !junctions.contains(&(entry.last_junction, entry.point)) {
                    junctions.insert((entry.last_junction, entry.point));
                    entry.apply_to(&mut edges);

                    for neighbour in neighbours {
                        exploring.push_back(ExploringEntry {
                            point: neighbour,
                            last_junction: entry.point,
                            came_from: Some(entry.point),
                            distance_to_last_junction: 1,
                        });
                    }
                }
            } else {
                // simple passage
                for neighbour in neighbours {
                    exploring.push_back(entry.next(neighbour));
                }
            }
        }

        TrailGraph {
            edges,
            start: trail.start,
            end: trail.end,
        }
    }

    fn find_longest_hike(&self) -> usize {
        let mut longest = 0;
        self.find_recursive(self.start, &mut HashSet::new(), 0, &mut longest);
        longest
    }

    fn find_recursive(
        &self,
        position: Point,
        seen: &mut HashSet<Point>,
        steps: usize,
        longest: &mut usize,
    ) {
        if position == self.end {
            *longest = (*longest).max(steps);
        } else if let Some(neighbours) = self.edges.get(&position) {
            for neighbour in neighbours {
                if !seen.contains(&neighbour.point) {
                    seen.insert(neighbour.point);
                    self.find_recursive(neighbour.point, seen, steps + neighbour.distance, longest);
                    seen.remove(&neighbour.point);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Trail {
    map: Grid<Tile>,
    start: Point,
    end: Point,
}

impl Trail {
    pub fn find_longest_hike(&self, slopes_are_slippy: bool) -> usize {
        TrailGraph::from(self, slopes_are_slippy).find_longest_hike()
    }

    /// Return all neighbours for a given point that are:
    /// - reachable from this point, and
    /// - not a forest tile.
    fn get_neighbours(&self, (row, col): Point, slopes_are_slippy: bool) -> Vec<Point> {
        let mut neighbours = Vec::with_capacity(4);
        let tile = self.map[(row, col)];

        if row > 0 && tile.can_go_north(slopes_are_slippy) {
            neighbours.push((row - 1, col));
        }

        if row + 1 < self.map.rows() && tile.can_go_south(slopes_are_slippy) {
            neighbours.push((row + 1, col));
        }

        if col > 0 && tile.can_go_west(slopes_are_slippy) {
            neighbours.push((row, col - 1));
        }

        if col + 1 < self.map.cols() && tile.can_go_east(slopes_are_slippy) {
            neighbours.push((row, col + 1));
        }

        neighbours
            .iter()
            .copied()
            .filter(|&p| !self.map[p].is_forest())
            .collect()
    }
}

impl FromInput for Trail {
    fn parse<T>(input: T) -> ParseResult<Self>
    where
        T: Iterator<Item = String>,
    {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut map = Grid::new(0, 0);

        for (row_idx, line) in input.enumerate() {
            let mut row: Vec<Tile> = Vec::with_capacity(line.len());
            for (col_idx, ch) in line.chars().enumerate() {
                row.push(match ch {
                    '#' => Tile::Forest,
                    '^' => Tile::SlopeNorth,
                    '>' => Tile::SlopeEast,
                    'v' => Tile::SlopeSouth,
                    '<' => Tile::SlopeWest,
                    _ => {
                        if row_idx == 0 {
                            start = (row_idx, col_idx);
                        } else {
                            end = (row_idx, col_idx);
                        }
                        Tile::Path
                    }
                });
            }
            map.push_row(row);
        }

        Ok(Trail { map, start, end })
    }
}

#[derive(Debug, Copy, Clone, Default)]
enum Tile {
    #[default]
    Path,
    Forest,
    SlopeNorth,
    SlopeEast,
    SlopeSouth,
    SlopeWest,
}

impl Tile {
    fn can_go_north(&self, slopes_are_slippy: bool) -> bool {
        !slopes_are_slippy || matches![self, Tile::Path | Tile::SlopeNorth]
    }

    fn can_go_south(&self, slopes_are_slippy: bool) -> bool {
        !slopes_are_slippy || matches![self, Tile::Path | Tile::SlopeSouth]
    }

    fn can_go_east(&self, slopes_are_slippy: bool) -> bool {
        !slopes_are_slippy || matches![self, Tile::Path | Tile::SlopeEast]
    }

    fn can_go_west(&self, slopes_are_slippy: bool) -> bool {
        !slopes_are_slippy || matches![self, Tile::Path | Tile::SlopeWest]
    }

    fn is_forest(&self) -> bool {
        matches![self, Tile::Forest]
    }
}
