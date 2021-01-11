use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

use grid::Grid;
use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

use crate::{three_by_three, two_by_two};

pub type Mapping<T> = (Grid<T>, Grid<T>);

pub fn parse_input<R: Read>(readable: R) -> Result<Vec<Mapping<u8>>, ParseError> {
    let mut enhancement_rules = vec![];
    for line in BufReader::new(readable).lines() {
        let line = line?;
        let left_list;
        let right;
        match line.len() {
            20 => {
                left_list = find(
                    parse_grid(&line[..5], 2)?,
                    two_by_two::rotate,
                    two_by_two::flip,
                );
                right = parse_grid(&line[9..], 3)?;
            }
            34 => {
                left_list = find(
                    parse_grid(&line[..11], 3)?,
                    three_by_three::rotate,
                    three_by_three::flip,
                );
                right = parse_grid(&line[15..], 4)?;
            }
            _ => {
                return Err(parse_error!("Invalid input line: {}", line));
            }
        }
        for left in left_list {
            enhancement_rules.push((left.clone(), right.clone()));
        }
    }
    Ok(enhancement_rules)
}

fn parse_grid(s: &str, cols: usize) -> Result<Grid<u8>, ParseError> {
    let mut fields = vec![];

    for ch in s.chars() {
        match ch {
            '.' => fields.push(0),
            '#' => fields.push(1),
            '/' => {}
            _ => {
                return Err(parse_error!("Invalid grid: {}", s));
            }
        }
    }

    Ok(Grid::from_vec(fields, cols))
}

pub fn find<R, F>(mut grid: Grid<u8>, rotate: R, flip: F) -> Vec<Grid<u8>>
where
    R: Fn(Grid<u8>) -> Grid<u8>,
    F: Fn(Grid<u8>) -> Grid<u8>,
{
    let mut v = vec![];

    v.push(grid.clone()); // original grid
    grid = rotate(grid);
    v.push(grid.clone()); // rotated 1x
    grid = rotate(grid);
    v.push(grid.clone()); // rotated 2x
    grid = rotate(grid);
    v.push(grid.clone()); // rotated 3x
    grid = flip(rotate(grid));
    v.push(grid.clone()); // flipped
    grid = rotate(grid);
    v.push(grid.clone()); // flipped & rotated 1x
    grid = rotate(grid);
    v.push(grid.clone()); // flipped & rotated 2x
    grid = rotate(grid);
    v.push(grid); // flipped & rotated 3x

    dedupe(v)
}

pub fn dedupe(list: Vec<Grid<u8>>) -> Vec<Grid<u8>> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut deduped = vec![];
    for g in list {
        let key = format!("{:?}", g);
        if !seen.contains(&key) {
            seen.insert(key);
            deduped.push(g);
        }
    }
    deduped
}
