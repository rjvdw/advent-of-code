use std::fmt;

use helpers::FromMultilineStr;
use Position::{Floor, Seat};
use SeatState::{Empty, Occupied};

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
            let nr_of_adjacent_occupied_seats = self
                .adjacent_seats(x, y, view_distance)
                .iter()
                .filter(|&&s| s == Seat(Occupied))
                .count();

            if nr_of_adjacent_occupied_seats == 0 {
                Seat(Occupied)
            } else if nr_of_adjacent_occupied_seats >= seat_threshold {
                Seat(Empty)
            } else {
                current
            }
        }
    }

    /// Find adjacent seats from a given seat, limited to `view_distance`. A `view_distance` of 0
    /// means unlimited.
    fn adjacent_seats(&self, x: usize, y: usize, view_distance: usize) -> Vec<Position> {
        let view_distance = if view_distance == 0 {
            usize::MAX
        } else {
            view_distance
        };
        let mut positions = Vec::new();

        // searching up from the current position
        #[rustfmt::skip]
        let option = self.find_next_seat(
            view_distance,
            |i| x >= i,
            |i| (x - i, y),
        );
        if let Some(seat) = option {
            positions.push(seat);
        }

        // searching diagonally up and to the left from the current position
        #[rustfmt::skip]
        let option = self.find_next_seat(
            view_distance,
            |i| x >= i && y >= i,
            |i| (x - i, y - i),
        );
        if let Some(seat) = option {
            positions.push(seat);
        }

        // searching to the left from the current position
        #[rustfmt::skip]
        let option = self.find_next_seat(
            view_distance,
            |i| y >= i,
            |i| (x, y - i),
        );
        if let Some(seat) = option {
            positions.push(seat);
        }

        // searching diagonally down and to the left from the current position
        #[rustfmt::skip]
        let option = self.find_next_seat(
            view_distance,
            |i| x + i < self.height && y >= i,
            |i| (x + i, y - i),
        );
        if let Some(seat) = option {
            positions.push(seat);
        }

        // searching down from the current position
        #[rustfmt::skip]
        let option = self.find_next_seat(
            view_distance,
            |i| x + i < self.height,
            |i| (x + i, y),
        );
        if let Some(seat) = option {
            positions.push(seat);
        }

        // searching diagonally down and to the right from the current position
        #[rustfmt::skip]
        let option = self.find_next_seat(
            view_distance,
            |i| x + i < self.height && y + i < self.width,
            |i| (x + i, y + i),
        );
        if let Some(seat) = option {
            positions.push(seat);
        }

        // searching to the right from the current position
        #[rustfmt::skip]
        let option = self.find_next_seat(
            view_distance,
            |i| y + i < self.width,
            |i| (x, y + i),
        );
        if let Some(seat) = option {
            positions.push(seat);
        }

        // searching diagonally up and to the right from the current position
        #[rustfmt::skip]
        let option = self.find_next_seat(
            view_distance,
            |i| x >= i && y + i < self.width,
            |i| (x - i, y + i),
        );
        if let Some(seat) = option {
            positions.push(seat);
        }

        positions
    }

    fn find_next_seat(
        &self,
        view_distance: usize,
        predicate: impl Fn(usize) -> bool,
        mapper: impl Fn(usize) -> (usize, usize),
    ) -> Option<Position> {
        (1..=view_distance)
            .take_while(|&i| predicate(i))
            .map(|i| mapper(i))
            .map(|(x, y)| self.current_state(x, y))
            .find(|&pos| pos == Seat(Occupied) || pos == Seat(Empty))
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

#[derive(Debug)]
pub struct SeatLayoutError {
    msg: String,
}

impl fmt::Display for SeatLayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl FromMultilineStr for SeatLayout {
    type Err = SeatLayoutError;

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
            return Err(SeatLayoutError {
                msg: format!(
                    "The width of the lines is not consistent (seen both {} and {}).",
                    self.width,
                    line.len()
                ),
            });
        }

        for char in line.chars() {
            self.seats.push(match char {
                '.' => Floor,
                'L' => Seat(Empty),
                '#' => Seat(Occupied),
                _ => {
                    return Err(SeatLayoutError {
                        msg: format!("Invalid input character found in line '{}'.", char),
                    });
                }
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::get_input;

    use super::*;

    #[test]
    fn test_1() {
        #[rustfmt::skip]
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

        assert_eq!(count_occupied_seats(state, 4, 3, 0), 8);
    }

    #[test]
    fn test_2() {
        #[rustfmt::skip]
        let state = get_input(vec![
            ".............",
            ".L.L.#.#.#.#.",
            ".............",
        ]);

        assert_eq!(count_occupied_seats(state, 1, 2, 0), 0);
    }

    #[test]
    fn test_3() {
        #[rustfmt::skip]
        let state = get_input(vec![
            ".##.##.",
            "#.#.#.#",
            "##...##",
            "...L...",
            "##...##",
            "#.#.#.#",
            ".##.##.",
        ]);

        assert_eq!(count_occupied_seats(state, 3, 3, 0), 0);
    }

    fn count_occupied_seats(state: SeatLayout, x: usize, y: usize, view_distance: usize) -> usize {
        state
            .adjacent_seats(x, y, view_distance)
            .iter()
            .filter(|&&seat| seat == Seat(Occupied))
            .count()
    }
}
