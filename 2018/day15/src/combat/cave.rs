use std::collections::{HashMap, HashSet};
use std::fmt;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::MultilineFromStr;

use crate::combat::faction::Faction;
use crate::combat::point::{cmp_points, compute_distance};
use crate::combat::state::State;
use crate::combat::tile::Tile;
use crate::combat::unit::Unit;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cave {
    layout: Vec<Vec<Tile>>,
    units: Vec<Unit>,
    width: usize,
    height: usize,
    turns: usize,
}

impl Cave {
    /// Define the attack power for a faction.
    pub fn set_attack_power(&mut self, faction: Faction, attack_power: usize) {
        for unit in self.units.iter_mut() {
            if unit.faction == faction {
                unit.attack_power = attack_power;
            }
        }
    }

    /// Get the number of remaining units for a faction.
    pub fn get_remaining_units_count(&self, faction: Faction) -> usize {
        self.units
            .iter()
            .filter(|u| u.is_alive() && u.faction == faction)
            .count()
    }

    /// Progresses the entire cave one turn.
    pub fn take_turns(&mut self) -> Option<(Faction, usize)> {
        self.units.sort_unstable();
        for i in 0..self.units.len() {
            let mut unit = self.units[i];
            if unit.is_alive() {
                if let State::Over = self.turn_move(&unit) {
                    return self.get_outcome();
                }
                unit = self.units[i];
                self.turn_attack(&unit);
            }
        }
        self.turns += 1;
        None
    }

    /// Determines the outcome of the battle.
    pub fn get_outcome(&self) -> Option<(Faction, usize)> {
        let mut faction_iter = self
            .units
            .iter()
            .filter(|u| u.is_alive())
            .map(|u| u.faction);

        if let Some(faction) = faction_iter.next() {
            if faction_iter.any(|f| f != faction) {
                // combat is not yet over
                None
            } else {
                Some((faction, self.turns * self.get_total_health(faction)))
            }
        } else {
            // everyone is dead, let's give the victory to the goblins
            Some((Faction::Goblin, 0))
        }
    }

    /// Gets the total health of all units within a faction.
    fn get_total_health(&self, faction: Faction) -> usize {
        self.units
            .iter()
            .filter(|u| u.faction == faction)
            .map(|u| u.health)
            .sum()
    }

    /// Part one of a units turn: Move.
    fn turn_move(&mut self, unit: &Unit) -> State {
        let (targets, is_adjacent_to_target) = self.find_targets(unit);

        // If the opposing faction has no units left, the combat is over.
        if targets.is_empty() {
            return State::Over;
        }

        // If we are not adjacent to a target, we are going to do a move.
        if !is_adjacent_to_target {
            // Determine the location of friendly units, as we may not pass through them.
            let friendlies = self.get_friendlies(unit.faction);

            // If we can find a target, move the unit towards this target.
            if let Some((target, distance)) = self.find_target(unit, &targets, &friendlies) {
                let next_position =
                    self.determine_next_position(unit, target, distance, &friendlies);

                if let Some(u) = self
                    .units
                    .iter_mut()
                    .find(|u| u.is_alive() && u.position == unit.position)
                {
                    u.position = next_position;
                }
            }
        }

        State::InProgress
    }

    /// Part two of a units turn: Attack.
    fn turn_attack(&mut self, unit: &Unit) {
        let neighbours = self.get_neighbours(unit.position);
        let targets: Vec<Unit> = neighbours
            .iter()
            .map(|&c| {
                self.units
                    .iter()
                    .find(|u| u.is_alive() && u.position == c && unit.opposes(u))
            })
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .copied()
            .collect();

        let target_opt = targets
            .iter()
            .map(|u| u.health)
            .min()
            .and_then(|min_health| targets.iter().filter(|u| u.health == min_health).min());

        if let Some(target) = target_opt {
            let target = self
                .units
                .iter_mut()
                .find(|u| u.is_alive() && u.position == target.position)
                .unwrap();

            target.take_damage(unit.attack_power);
        }
    }

    /// Find all potential targets, and checks if any target is adjacent to this unit.
    fn find_targets(&self, unit: &Unit) -> (Vec<Unit>, bool) {
        let mut result = (Vec::new(), false);
        for &other in self
            .units
            .iter()
            .filter(|u| u.is_alive() && u.opposes(unit))
        {
            result.0.push(other);
            if !result.1 && compute_distance(unit.position, other.position) == 1 {
                result.1 = true;
            }
        }
        result
    }

    /// Determine all friendly units in the cave.
    fn get_friendlies(&self, faction: Faction) -> Vec<Unit> {
        self.units
            .iter()
            .filter(|u| u.is_alive() && u.faction == faction)
            .copied()
            .collect()
    }

    /// Find the closest target, and the distance to this target.
    fn find_target(
        &self,
        unit: &Unit,
        targets: &[Unit],
        friendlies: &[Unit],
    ) -> Option<(Unit, usize)> {
        // We are going to do a flood fill to find potential targets.

        // This contains the points that we are currently exploring.
        let mut exploring: HashSet<(usize, usize)> = HashSet::new();
        exploring.insert(unit.position);

        // This contains all the points that we have already explored. We are going to add in
        // all the friendlies as well, so that we do not pass through them.
        let mut explored: HashSet<(usize, usize)> = HashSet::new();
        explored.insert(unit.position);
        explored.extend(friendlies.iter().map(|u| u.position));

        // This will contain the distance between us and the target.
        let mut distance = 0;

        // For easier searching, create a map of targets and their positions.
        let mut target_positions = HashMap::with_capacity(targets.len());
        target_positions.extend(targets.iter().map(|u| (u.position, *u)));

        while !exploring.is_empty() {
            distance += 1;

            let mut next_exploring = HashSet::new();
            let mut potential_targets = HashSet::new();

            // For every point we are currently exploring, find its neighbours and determine if
            // they are either a potential target, or else if they are already explored.
            for point in exploring {
                for neighbour in self.get_neighbours(point) {
                    if target_positions.contains_key(&neighbour) {
                        potential_targets.insert(neighbour);
                    } else if !explored.contains(&neighbour) {
                        next_exploring.insert(neighbour);
                    }
                    explored.insert(neighbour);
                }
            }

            exploring = next_exploring;

            // If we have found potential targets, select the first one.
            if !potential_targets.is_empty() {
                return potential_targets
                    .iter()
                    .min_by(|a, b| cmp_points(a, b))
                    .and_then(|p| target_positions.get(p))
                    .copied()
                    .map(|p| (p, distance));
            }
        }

        None
    }

    /// Determine the next position this unit will move to, given a target.
    fn determine_next_position(
        &self,
        unit: &Unit,
        target: Unit,
        distance: usize,
        friendlies: &[Unit],
    ) -> (usize, usize) {
        // In order to find the direction to move to, we are again going to do a flood fill. This
        // time we are going to do this for the specified distance - 1. We then check which of the
        // tiles we've reached are adjacent to the unit, and from these tiles we take the first one
        // (according to the specified order).

        let mut exploring = HashSet::new();
        exploring.insert(target.position);

        let mut explored = HashSet::new();
        explored.insert(target.position);
        explored.extend(friendlies.iter().map(|u| u.position));

        let mut d2 = 1;

        while d2 < distance {
            d2 += 1;

            let mut next_exploring = HashSet::new();

            for point in exploring {
                for neighbour in self.get_neighbours(point) {
                    if !explored.contains(&neighbour) {
                        next_exploring.insert(neighbour);
                    }
                }
            }

            exploring = next_exploring;
        }

        exploring
            .iter()
            .filter(|&&p| compute_distance(p, unit.position) == 1)
            .min_by(|a, b| cmp_points(a, b))
            .copied()
            .unwrap()
    }

    /// Find all empty tiles (i.e. not containing a wall) next to a given point.
    fn get_neighbours(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        if x > 0 {
            neighbours.push((x - 1, y))
        }
        if x + 1 < self.width {
            neighbours.push((x + 1, y))
        }
        if y > 0 {
            neighbours.push((x, y - 1))
        }
        if y + 1 < self.height {
            neighbours.push((x, y + 1))
        }

        neighbours
            .iter()
            .filter(|&&(x, y)| self.layout[y][x].is_empty())
            .copied()
            .collect()
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (y, row) in self.layout.iter().enumerate() {
            if y != 0 {
                writeln!(f)?;
            }
            for (x, tile) in row.iter().enumerate() {
                if let Some(unit) = self
                    .units
                    .iter()
                    .find(|u| u.is_alive() && u.position == (x, y))
                {
                    match unit.faction {
                        Faction::Elf => write!(f, "E")?,
                        Faction::Goblin => write!(f, "G")?,
                    }
                } else {
                    match tile {
                        Tile::Wall => write!(f, "#")?,
                        Tile::Empty => write!(f, ".")?,
                    }
                }
            }
        }
        Ok(())
    }
}

impl MultilineFromStr for Cave {
    type Err = ParseError;

    fn new() -> Self {
        Cave {
            layout: Vec::new(),
            units: Vec::new(),
            width: 0,
            height: 0,
            turns: 0,
        }
    }

    fn indicates_new_record(&self, _line: &str) -> bool {
        false
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        self.height += 1;
        self.width = line.len();
        let mut row = Vec::new();
        let y = self.layout.len();
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => row.push(Tile::Wall),
                '.' => row.push(Tile::Empty),
                'G' => {
                    row.push(Tile::Empty);
                    self.units.push(Unit::new(Faction::Goblin, (x, y)));
                }
                'E' => {
                    row.push(Tile::Empty);
                    self.units.push(Unit::new(Faction::Elf, (x, y)));
                }
                _ => return Err(ParseError(format!("Invalid tile: {}", ch))),
            }
        }
        self.layout.push(row);
        Ok(())
    }
}

#[cfg(test)]
#[rustfmt::skip::macros(vec)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsMultilineRecords;

    use super::*;

    #[test]
    fn test_movement() {
        let mut cave = get_test_input(vec![
            "#########",
            "#G..G..G#",
            "#.......#",
            "#.......#",
            "#G..E..G#",
            "#.......#",
            "#.......#",
            "#G..G..G#",
            "#########",
        ]);

        cave.take_turns();
        assert_eq!(
            format!("{}", cave),
            vec![
                "#########",
                "#.G...G.#",
                "#...G...#",
                "#...E..G#",
                "#.G.....#",
                "#.......#",
                "#G..G..G#",
                "#.......#",
                "#########",
            ]
            .join("\n")
        );

        cave.take_turns();
        assert_eq!(
            format!("{}", cave),
            vec![
                "#########",
                "#..G.G..#",
                "#...G...#",
                "#.G.E.G.#",
                "#.......#",
                "#G..G..G#",
                "#.......#",
                "#.......#",
                "#########",
            ]
            .join("\n")
        );

        cave.take_turns();
        assert_eq!(
            format!("{}", cave),
            vec![
                "#########",
                "#.......#",
                "#..GGG..#",
                "#..GEG..#",
                "#G..G...#",
                "#......G#",
                "#.......#",
                "#.......#",
                "#########",
            ]
            .join("\n")
        );
    }

    #[test]
    fn test_attack() {
        let mut cave = get_test_input(vec![
            "#######",
            "#.G...#",
            "#...EG#",
            "#.#.#G#",
            "#..G#E#",
            "#.....#",
            "#######",
        ]);

        // after 23 turns, the first unit falls
        for _ in 0..23 {
            cave.take_turns();
        }

        assert_eq!(
            format!("{}", cave),
            vec![
                "#######",
                "#...G.#",
                "#..G.G#",
                "#.#.#G#",
                "#...#E#",
                "#.....#",
                "#######",
            ]
            .join("\n")
        );
    }

    fn get_test_input(lines: Vec<&str>) -> Cave {
        lines
            .as_multiline_records::<Cave>()
            .unwrap()
            .first()
            .unwrap()
            .clone()
    }
}
