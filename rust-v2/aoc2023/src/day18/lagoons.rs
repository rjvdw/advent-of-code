use crate::direction::Direction;
use crate::lagoon::Lagoon;
use crate::point::Point;
use crate::section::Section;
use rdcl_aoc_core::input::FromInput;
use rdcl_aoc_core::{err_parse_error, ParseResult};

#[derive(Debug)]
pub struct Lagoons {
    pub simple: Lagoon,
    pub correct: Lagoon,
}

impl FromInput for Lagoons {
    fn parse<T>(input: T) -> ParseResult<Self>
    where
        T: Iterator<Item = String>,
    {
        let mut simple = Lagoon::default();
        let mut simple_current = Point::new(0, 0);

        let mut correct = Lagoon::default();
        let mut correct_current = Point::new(0, 0);

        for line in input {
            let p = line.rfind(' ').ok_or(())?;

            let direction = match line.chars().next().ok_or(())? {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                v => {
                    return err_parse_error!("Invalid direction: {}", v);
                }
            };
            let count = line[2..p].parse::<usize>()?;

            let section = Section::new(simple_current, direction, count);
            simple.trenches.push(section);
            simple_current = section.to;
            simple.update_bounds(simple_current);

            let direction = match line.chars().nth(line.len() - 2).ok_or(())? {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                v => {
                    return err_parse_error!("Invalid direction: {}", v);
                }
            };
            let count = usize::from_str_radix(&line[p + 3..line.len() - 2], 16)?;

            let section = Section::new(correct_current, direction, count);
            correct.trenches.push(section);
            correct_current = section.to;
            correct.update_bounds(correct_current);
        }

        if simple_current != (0, 0) {
            eprintln!("lagoon:\n{simple}");
            eprintln!("rows: {} - {}", simple.min_row, simple.max_row);
            eprintln!("cols: {} - {}", simple.min_col, simple.max_col);
            err_parse_error!(
                "The trench did not loop back on itself, it ended at ({:?})",
                simple_current
            )
        } else if correct_current != (0, 0) {
            // eprintln!("lagoon:\n{correct}");
            eprintln!("rows: {} - {}", correct.min_row, correct.max_row);
            eprintln!("cols: {} - {}", correct.min_col, correct.max_col);
            err_parse_error!(
                "The trench did not loop back on itself, it ended at ({:?})",
                correct_current
            )
        } else {
            Ok(Lagoons { simple, correct })
        }
    }
}
