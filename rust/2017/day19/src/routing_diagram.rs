use std::fmt;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::MultilineFromStr;

use crate::line::Line;

type Grid<T> = Vec<Vec<T>>;

#[derive(Debug, Clone)]
pub struct RoutingDiagram {
    points: Grid<Line>,
}

impl RoutingDiagram {
    pub fn find_start(&self) -> Option<(usize, usize)> {
        self.points.get(1).and_then(|row| {
            row.iter()
                .enumerate()
                .find(|(_, &l)| l == Line::Vertical)
                .map(|(x, _)| (x, 1))
        })
    }

    pub fn at(&self, (x, y): (usize, usize)) -> Line {
        self.points
            .get(y)
            .and_then(|row| row.get(x))
            .copied()
            .unwrap_or(Line::Empty)
    }
}

impl fmt::Display for RoutingDiagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, row) in self.points.iter().enumerate() {
            if idx != 0 {
                writeln!(f)?;
            }
            for line in row {
                match line {
                    Line::Horizontal => write!(f, "-")?,
                    Line::Vertical => write!(f, "|")?,
                    Line::Junction => write!(f, "+")?,
                    Line::Empty => write!(f, " ")?,
                    Line::Letter(ch) => write!(f, "{}", ch)?,
                }
            }
        }

        Ok(())
    }
}

impl MultilineFromStr for RoutingDiagram {
    type Err = ParseError;

    fn new() -> Self {
        RoutingDiagram { points: vec![] }
    }

    fn indicates_new_record(&self, _line: &str) -> bool {
        false
    }

    fn parse(&mut self, s: &str) -> Result<(), Self::Err> {
        if self.points.is_empty() {
            self.points.push(vec![Line::Empty; s.len() + 2]);
        }

        let mut row = Vec::with_capacity(s.len() + 2);
        row.push(Line::Empty);

        for ch in s.chars() {
            let line = match ch {
                '-' => Line::Horizontal,
                '|' => Line::Vertical,
                '+' => Line::Junction,
                ' ' => Line::Empty,
                _ => Line::Letter(ch),
            };
            row.push(line);
        }

        self.points.push(row);
        Ok(())
    }
}
