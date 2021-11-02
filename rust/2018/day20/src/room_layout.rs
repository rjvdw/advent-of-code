use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::math::taxi_cab_2d;
use rdcl_aoc_helpers::parse_error;
use rdcl_aoc_helpers::search::Navigable;

type Room = (i64, i64);

#[derive(Debug)]
pub struct RoomLayout {
    doors: HashSet<(Room, Room)>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl RoomLayout {
    pub fn find_longest_path(&self) -> usize {
        let mut exploring: VecDeque<(usize, Room)> = VecDeque::new();
        exploring.push_back((0, (0, 0)));

        let mut explored: HashSet<Room> = HashSet::new();
        explored.insert((0, 0));

        let mut longest_distance = 0;
        while let Some((distance, room)) = exploring.pop_front() {
            for (d, neighbour) in self.get_neighbours(&room) {
                if !explored.contains(&neighbour) {
                    longest_distance = distance + d as usize;
                    exploring.push_back((longest_distance, neighbour));
                    explored.insert(neighbour);
                }
            }
        }
        longest_distance
    }

    pub fn find_paths_longer_than(&self, threshold: usize) -> usize {
        let mut exploring: VecDeque<(usize, Room)> = VecDeque::new();
        exploring.push_back((0, (0, 0)));

        let mut explored: HashSet<Room> = HashSet::new();
        explored.insert((0, 0));

        let mut count = 0;
        while let Some((distance, room)) = exploring.pop_front() {
            for (d, neighbour) in self.get_neighbours(&room) {
                if !explored.contains(&neighbour) {
                    let distance = distance + d as usize;
                    if distance > threshold {
                        count += 1;
                    }
                    exploring.push_back((distance, neighbour));
                    explored.insert(neighbour);
                }
            }
        }
        count
    }

    fn walk(
        &mut self,
        regex: &str,
        offset: usize,
        depth: usize,
        initial_exploring: &HashSet<Room>,
    ) -> Result<(usize, HashSet<Room>), ParseError> {
        let mut exploring = initial_exploring.clone();
        let mut end_points = HashSet::new();

        let mut skip = 0;
        for (idx, ch) in regex.chars().enumerate().skip(offset) {
            if skip > 0 {
                skip -= 1;
                continue;
            }

            let mut direction: Option<(i64, i64)> = None;
            match ch {
                'N' => {
                    direction = Some((0, -1));
                }
                'E' => {
                    direction = Some((1, 0));
                }
                'S' => {
                    direction = Some((0, 1));
                }
                'W' => {
                    direction = Some((-1, 0));
                }
                '(' => {
                    let (next_idx, next_exploring) =
                        self.walk(regex, idx + 1, depth + 1, &exploring)?;
                    exploring = next_exploring;
                    skip = next_idx - 1 - idx;
                }
                '|' => {
                    end_points.extend(exploring);
                    exploring = initial_exploring.clone();
                }
                ')' => {
                    return if depth == 0 {
                        Err(parse_error!("Illegal closing bracket encountered."))
                    } else {
                        Ok((idx + 1, end_points))
                    };
                }
                '$' => {
                    return if depth != 0 {
                        Err(parse_error!("Illegal dollar symbol encountered."))
                    } else {
                        Ok((idx + 1, end_points))
                    };
                }
                _ => return Err(parse_error!("Invalid character: {}", ch)),
            }

            if let Some(direction) = direction {
                let mut next_exploring = HashSet::new();
                for &room in &exploring {
                    let next_room = (room.0 + direction.0, room.1 + direction.1);
                    self.add_door(room, next_room);
                    self.expand_space(next_room);
                    next_exploring.insert(next_room);
                }
                exploring = next_exploring;
            }
        }

        Ok((regex.len(), end_points))
    }

    fn add_door(&mut self, from: Room, to: Room) {
        self.doors.insert((from, to));
        self.doors.insert((to, from));
    }

    fn expand_space(&mut self, (x, y): Room) {
        if x < self.min_x {
            self.min_x = x;
        }
        if x > self.max_x {
            self.max_x = x;
        }
        if y < self.min_y {
            self.min_y = y;
        }
        if y > self.max_y {
            self.max_y = y;
        }
    }
}

impl Navigable for RoomLayout {
    type Point = Room;

    fn distance_score(&self, a: &Self::Point, b: &Self::Point) -> u64 {
        taxi_cab_2d(*a, *b) as u64
    }

    fn get_neighbours(&self, point: &Self::Point) -> Vec<(u64, Self::Point)> {
        let mut neighbours = vec![*point, *point, *point, *point];
        neighbours[0].0 -= 1;
        neighbours[1].0 += 1;
        neighbours[2].1 -= 1;
        neighbours[3].1 += 1;

        neighbours
            .iter()
            .filter(|&p| self.doors.contains(&(*p, *point)))
            .copied()
            .map(|p| (1, p))
            .collect()
    }
}

impl fmt::Display for RoomLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                write!(f, "#")?;
                if self.doors.contains(&((x, y), (x, y - 1))) {
                    write!(f, "-")?;
                } else {
                    write!(f, "#")?;
                }
            }
            writeln!(f, "#")?;

            for x in self.min_x..=self.max_x {
                if self.doors.contains(&((x, y), (x - 1, y))) {
                    write!(f, "|")?;
                } else {
                    write!(f, "#")?;
                }
                if x == 0 && y == 0 {
                    write!(f, "X")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "#")?;
        }

        for _ in self.min_x..=self.max_x {
            write!(f, "##")?;
        }
        write!(f, "#")?;

        Ok(())
    }
}

impl FromStr for RoomLayout {
    type Err = ParseError;

    fn from_str(regex: &str) -> Result<Self, Self::Err> {
        let mut layout = RoomLayout {
            doors: HashSet::new(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        };
        let mut exploring = HashSet::new();
        exploring.insert((0, 0));
        layout.walk(regex, 1, 0, &exploring)?;
        Ok(layout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let mut layout = RoomLayout {
            doors: HashSet::new(),
            min_x: -2,
            max_x: 1,
            min_y: -2,
            max_y: 1,
        };

        layout.add_door((-2, -2), (-1, -2));
        layout.add_door((-1, -2), (0, -2));
        layout.add_door((0, -2), (1, -2));

        layout.add_door((-2, -2), (-2, -1));

        layout.add_door((-2, -1), (-1, -1));
        layout.add_door((-1, -1), (0, -1));
        layout.add_door((0, -1), (1, -1));

        layout.add_door((-2, -1), (-2, 0));
        layout.add_door((1, -1), (1, 0));

        layout.add_door((0, 0), (1, 0));

        layout.add_door((-2, 0), (-2, 1));
        layout.add_door((-1, 0), (-1, 1));

        layout.add_door((-2, 1), (-1, 1));
        layout.add_door((-1, 1), (0, 1));
        layout.add_door((0, 1), (1, 1));

        assert_eq!(
            format!("{}", layout),
            vec![
                "#########",
                "#.|.|.|.#",
                "#-#######",
                "#.|.|.|.#",
                "#-#####-#",
                "#.#.#X|.#",
                "#-#-#####",
                "#.|.|.|.#",
                "#########",
            ]
            .join("\n")
        );
    }
}
