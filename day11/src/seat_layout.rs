use std::fmt;

use helpers::from_multiline_str::FromMultilineStr;
use helpers::parse_error::ParseError;
use Position::{Floor, Seat};
use SeatState::{Empty, Occupied};

use crate::cardinal_direction;
use crate::cardinal_direction::CardinalDirection;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum SeatState {
    Empty,
    Occupied,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum Position {
    Floor,
    Seat(SeatState),
}

impl Position {
    const fn is_seat(&self) -> bool {
        matches!(*self, Seat(_))
    }
}

#[derive(Clone)]
pub struct SeatLayout {
    seats: Vec<Position>,
    width: usize,
    height: usize,
}

impl SeatLayout {
    fn new(width: usize, height: usize) -> SeatLayout {
        let seats = vec![Floor; width * height];
        SeatLayout {
            seats,
            width,
            height,
        }
    }

    /// Moves the seat layout to its next state, and returns whether this new state is different
    /// from the previous state. `view_distance` determines how far passengers will look to find
    /// seats. Set to 0 for unlimited distance.
    pub fn next(&self, view_distance: usize, seat_treshold: usize) -> (Self, bool) {
        let mut new_state = SeatLayout::new(self.width, self.height);
        let mut changed = false;

        for x in 0..self.height {
            for y in 0..self.width {
                let current = self.current_state(x, y);
                let next = self.next_state(x, y, view_distance, seat_treshold);

                if current != next {
                    changed = true
                }

                new_state.set_state(x, y, next);
            }
        }

        (new_state, changed)
    }

    /// Count the number of occupied seats.
    pub fn nr_of_occupied_seats(&self) -> usize {
        self.seats.iter().filter(|&&v| v == Seat(Occupied)).count()
    }

    /// Determines the current state at a specific position.
    fn current_state(&self, x: usize, y: usize) -> Position {
        self.seats[x * self.width + y]
    }

    /// Determines the next state at a specific position.
    fn next_state(
        &self,
        x: usize,
        y: usize,
        view_distance: usize,
        seat_threshold: usize,
    ) -> Position {
        let current = self.current_state(x, y);
        if current == Floor {
            current
        } else {
            let nr_of_adjacent_occupied_seats =
                self.count_occupied_adjacent_seats(view_distance, x, y);
            if nr_of_adjacent_occupied_seats == 0 {
                Seat(Occupied)
            } else if nr_of_adjacent_occupied_seats >= seat_threshold {
                Seat(Empty)
            } else {
                current
            }
        }
    }

    /// Count the number of adjacent seats that are occupied.
    fn count_occupied_adjacent_seats(&self, view_distance: usize, x: usize, y: usize) -> usize {
        cardinal_direction::ALL_DIRECTIONS
            .iter()
            .map(|&d| self.find_next_seat(view_distance, d, x, y))
            .filter(|&o| o == Some(Seat(Occupied)))
            .count()
    }

    /// Find adjacent seat from a given set, in the specified `direction`, limited to
    /// `view_distance`. A `view_distance` of 0 means unlimited.
    fn find_next_seat(
        &self,
        view_distance: usize,
        direction: CardinalDirection,
        x: usize,
        y: usize,
    ) -> Option<Position> {
        let view_distance = if view_distance == 0 {
            usize::MAX
        } else {
            view_distance
        };

        (1..=view_distance)
            .take_while(|&i| direction.predicate(x, y, self.height, self.width, i))
            .map(|i| direction.mapper(x, y, i))
            .map(|(x, y)| self.current_state(x, y))
            .find(|&pos| pos.is_seat())
    }

    /// Updates a specific position.
    fn set_state(&mut self, x: usize, y: usize, value: Position) {
        self.seats[x * self.width + y] = value;
    }
}

impl fmt::Display for SeatLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in 0..self.height {
            for y in 0..self.width {
                write!(
                    f,
                    "{}",
                    match self.current_state(x, y) {
                        Floor => '.',
                        Seat(Empty) => 'L',
                        Seat(Occupied) => '#',
                    }
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl FromMultilineStr for SeatLayout {
    const DISCARD_FIRST_RECORD: bool = false;

    type Err = ParseError;

    fn new() -> Self {
        SeatLayout::new(0, 0)
    }

    fn indicates_new_record(_line: &str) -> bool {
        false
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        self.height += 1;
        if self.width == 0 {
            self.width = line.len();
        } else if self.width != line.len() {
            return Err(ParseError(format!(
                "The width of the lines is not consistent (seen both {} and {}).",
                self.width,
                line.len()
            )));
        }

        for char in line.chars() {
            self.seats.push(match char {
                '.' => Ok(Floor),
                'L' => Ok(Seat(Empty)),
                '#' => Ok(Seat(Occupied)),
                _ => Err(ParseError(format!(
                    "Invalid input character found in line '{}'.",
                    char,
                ))),
            }?);
        }

        Ok(())
    }
}

#[cfg(test)]
#[rustfmt::skip::macros(vec)]
mod tests {
    use crate::test_helpers::get_input;

    #[test]
    fn test_1() {
        let state = get_input(vec![
            ".......#.",
            "...#.....",
            ".#.......",
            ".........",
            "..#L....#",
            "....#....",
            ".........",
            "#........",
            "...#.....",
        ]);

        assert_eq!(state.count_occupied_adjacent_seats(0, 4, 3), 8);
    }

    #[test]
    fn test_2() {
        let state = get_input(vec![
            ".............",
            ".L.L.#.#.#.#.",
            ".............",
        ]);

        assert_eq!(state.count_occupied_adjacent_seats(0, 1, 2), 0);
    }

    #[test]
    fn test_3() {
        let state = get_input(vec![
            ".##.##.",
            "#.#.#.#",
            "##...##",
            "...L...",
            "##...##",
            "#.#.#.#",
            ".##.##.",
        ]);

        assert_eq!(state.count_occupied_adjacent_seats(0, 3, 3), 0);
    }
}
