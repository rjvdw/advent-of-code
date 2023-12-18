use std::collections::HashSet;
use std::fmt;

use crate::point::Point;
use crate::section::Section;

#[derive(Debug, Clone, Default)]
pub struct Lagoon {
    pub trenches: Vec<Section>,
    pub min_row: i32,
    pub max_row: i32,
    pub min_col: i32,
    pub max_col: i32,
}

impl Lagoon {
    pub fn size(&self) -> usize {
        let mut count = 0;
        let interesting_rows = self.interesting_rows();
        let interesting_columns = self.interesting_columns();

        let mut prev_row = self.min_row;
        let mut row_count = 0usize;

        for row in interesting_rows {
            count += row_count * (row - prev_row) as usize;
            row_count = 0;

            let mut ups = 0usize;
            let mut downs = 0usize;
            let mut prev_col = self.min_col;

            for &col in &interesting_columns {
                if let Some((point, idx)) = self.is_edge(row, col) {
                    row_count += (col - prev_col) as usize;
                    if self.edge_goes_down(point, idx) {
                        downs += 1;
                    }
                    if self.edge_goes_up(point, idx) {
                        ups += 1;
                    }
                } else if ups % 2 == 1 && downs % 2 == 1 {
                    row_count += (col - prev_col) as usize;
                }
                prev_col = col;
            }
            prev_row = row;
        }

        count + row_count
    }

    pub fn update_bounds(&mut self, Point { row, col }: Point) {
        self.min_row = self.min_row.min(row);
        self.max_row = self.max_row.max(row);
        self.min_col = self.min_col.min(col);
        self.max_col = self.max_col.max(col);
    }

    fn is_edge(&self, row: i32, col: i32) -> Option<(Point, usize)> {
        let point = Point::new(row, col);
        let mut edge = None;
        for (idx, section) in self.trenches.iter().enumerate() {
            if section.contains(point) {
                if section.is_vertical() {
                    return Some((point, idx));
                }
                edge = Some((point, idx))
            }
        }
        edge
    }

    fn edge_goes_down(&self, point: Point, idx: usize) -> bool {
        self.trenches[idx].goes_down(point)
    }

    fn edge_goes_up(&self, point: Point, idx: usize) -> bool {
        self.trenches[idx].goes_up(point)
    }

    fn interesting_rows(&self) -> Vec<i32> {
        let mut rows = self
            .trenches
            .iter()
            .flat_map(|trench| {
                [
                    trench.from.row - 1,
                    trench.from.row,
                    trench.from.row + 1,
                    trench.to.row - 1,
                    trench.to.row,
                    trench.to.row + 1,
                ]
            })
            .collect::<HashSet<_>>()
            .iter()
            .copied()
            .collect::<Vec<_>>();

        rows.sort();

        rows
    }

    fn interesting_columns(&self) -> Vec<i32> {
        let mut columns = self
            .trenches
            .iter()
            .flat_map(|trench| {
                [
                    trench.from.col - 1,
                    trench.from.col,
                    trench.from.col + 1,
                    trench.to.col - 1,
                    trench.to.col,
                    trench.to.col + 1,
                ]
            })
            .collect::<HashSet<_>>()
            .iter()
            .copied()
            .collect::<Vec<_>>();

        columns.sort();

        columns
    }
}

impl fmt::Display for Lagoon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for row in self.min_row..=self.max_row {
            if !first {
                writeln!(f)?;
            }
            for col in self.min_col..=self.max_col {
                if self.is_edge(row, col).is_some() {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            first = false;
        }

        Ok(())
    }
}
