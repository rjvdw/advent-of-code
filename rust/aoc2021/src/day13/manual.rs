use std::collections::HashSet;
use std::{fmt, io};

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

#[derive(Copy, Clone)]
pub enum Fold {
    Up(i32),
    Left(i32),
}

impl fmt::Debug for Fold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fold::Up(y) => write!(f, "Up({})", y),
            Fold::Left(x) => write!(f, "Left({})", x),
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Dot {
    pub x: i32,
    pub y: i32,
}

impl fmt::Debug for Dot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub struct Manual {
    dots: HashSet<Dot>,
    folds: Vec<Fold>,
    pointer: usize,
}

impl Manual {
    pub fn fold(&mut self) {
        self.dots = self
            .dots
            .iter()
            .map(|dot| match self.folds[self.pointer] {
                Fold::Up(y) if dot.y > y => Dot {
                    x: dot.x,
                    y: y - (dot.y - y),
                },
                Fold::Left(x) if dot.x > x => Dot {
                    x: x - (dot.x - x),
                    y: dot.y,
                },
                _ => *dot,
            })
            .collect();
        self.pointer += 1;
    }

    pub fn nr_folds(&self) -> usize {
        self.folds.len() - self.pointer
    }

    pub fn count_visible_dots(&self) -> usize {
        self.dots.len()
    }

    pub fn parse<I>(lines: I) -> Result<Manual, ParseError>
    where
        I: Iterator<Item = io::Result<String>>,
    {
        let mut manual = Manual {
            dots: HashSet::new(),
            folds: vec![],
            pointer: 0,
        };

        for line in lines {
            let line = line?;

            if let Some(p) = line.find(',') {
                let x = line[..p].parse()?;
                let y = line[p + 1..].parse()?;
                manual.dots.insert(Dot { x, y });
            } else if let Some(y) = line.strip_prefix("fold along y=") {
                manual.folds.push(Fold::Up(y.parse()?));
            } else if let Some(x) = line.strip_prefix("fold along x=") {
                manual.folds.push(Fold::Left(x.parse()?));
            } else if !line.is_empty() {
                return Err(parse_error!("Invalid line encountered: {}", line));
            }
        }

        Ok(manual)
    }
}

impl fmt::Display for Manual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        for dot in &self.dots {
            if dot.x < min_x {
                min_x = dot.x;
            }
            if dot.x > max_x {
                max_x = dot.x;
            }
            if dot.y < min_y {
                min_y = dot.y;
            }
            if dot.y > max_y {
                max_y = dot.y;
            }
        }

        for y in min_y..=max_y {
            if y != min_y {
                writeln!(f)?;
            }
            for x in min_x..=max_x {
                if self.dots.contains(&Dot { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold() {
        let mut manual = get_test_data();
        assert_eq!(manual.count_visible_dots(), 18);

        manual.fold();
        assert_eq!(manual.count_visible_dots(), 17);
    }

    #[test]
    fn test_display() {
        let mut manual = get_test_data();
        while manual.nr_folds() > 0 {
            manual.fold();
        }
        assert_eq!(
            format!("{}", manual),
            vec![
                "#####".to_string(),
                "#...#".to_string(),
                "#...#".to_string(),
                "#...#".to_string(),
                "#####".to_string(),
            ]
            .join("\n"),
        )
    }

    fn get_test_data() -> Manual {
        let lines = vec![
            Ok("6,10".to_string()),
            Ok("0,14".to_string()),
            Ok("9,10".to_string()),
            Ok("0,3".to_string()),
            Ok("10,4".to_string()),
            Ok("4,11".to_string()),
            Ok("6,0".to_string()),
            Ok("6,12".to_string()),
            Ok("4,1".to_string()),
            Ok("0,13".to_string()),
            Ok("10,12".to_string()),
            Ok("3,4".to_string()),
            Ok("3,0".to_string()),
            Ok("8,4".to_string()),
            Ok("1,10".to_string()),
            Ok("2,14".to_string()),
            Ok("8,10".to_string()),
            Ok("9,0".to_string()),
            Ok("".to_string()),
            Ok("fold along y=7".to_string()),
            Ok("fold along x=5".to_string()),
        ];

        Manual::parse(lines.into_iter()).unwrap()
    }
}
