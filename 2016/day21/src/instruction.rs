use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::err_parse_error;
use rdcl_aoc_helpers::error::ParseError;

#[derive(Clone)]
pub enum Instruction {
    SwapPositions(usize, usize),
    SwapLetters(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBased(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Instruction {
    pub fn run(&self, chars: &mut [char]) {
        match self {
            Instruction::SwapPositions(a, b) => {
                chars.swap(*a, *b);
            }
            Instruction::SwapLetters(ch1, ch2) => {
                let find_a = chars.iter().position(|c| *c == *ch1);
                let find_b = chars.iter().position(|c| *c == *ch2);
                if let (Some(a), Some(b)) = (find_a, find_b) {
                    chars.swap(a, b);
                }
            }
            Instruction::RotateLeft(i) => {
                let cl = chars.len();
                let i = *i % cl;
                if i > 0 {
                    for idx in 0..cl - i {
                        chars.swap(idx, idx + i)
                    }

                    // FIXME: there is probably a more efficient way of doing this
                    for _ in 0..cl % i {
                        for idx in 1..i {
                            chars.swap(cl - idx, cl - idx - 1)
                        }
                    }
                }
            }
            Instruction::RotateRight(i) => {
                Instruction::RotateLeft(chars.len() - (*i % chars.len())).run(chars);
            }
            Instruction::RotateBased(ch) => {
                let find_ch = chars.iter().position(|c| *c == *ch);
                if let Some(mut idx) = find_ch {
                    if idx >= 4 {
                        idx += 1;
                    }
                    Instruction::RotateRight(idx + 1).run(chars);
                }
            }
            Instruction::Reverse(a, b) => {
                for i in 0..(b - a + 1) / 2 {
                    chars.swap(a + i, b - i);
                }
            }
            Instruction::Move(a, b) => {
                if a < b {
                    for i in *a..*b {
                        chars.swap(i, i + 1);
                    }
                } else {
                    for i in 0..a - b {
                        chars.swap(a - i, a - i - 1);
                    }
                }
            }
        }
    }

    pub fn reverse(&self, chars: &mut [char]) {
        match self {
            Instruction::RotateLeft(i) => Instruction::RotateRight(*i).run(chars),
            Instruction::RotateRight(i) => Instruction::RotateLeft(*i).run(chars),
            Instruction::RotateBased(ch) => {
                let find_ch = chars.iter().position(|c| *c == *ch);
                if let Some(idx) = find_ch {
                    let rotate = match idx {
                        0 => Instruction::RotateLeft(1),  // -> 7
                        1 => Instruction::RotateLeft(1),  // -> 0
                        2 => Instruction::RotateRight(2), // -> 4
                        3 => Instruction::RotateLeft(2),  // -> 1
                        4 => Instruction::RotateRight(1), // -> 5
                        5 => Instruction::RotateLeft(3),  // -> 2
                        6 => Instruction::RotateRight(0), // -> 6
                        7 => Instruction::RotateLeft(4),  // -> 3
                        _ => panic!("Can only reverse engineer passwords with a length of 8."),
                    };
                    rotate.run(chars);
                }
            }
            Instruction::Move(a, b) => Instruction::Move(*b, *a).run(chars),
            _ => self.run(chars),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::SwapPositions(a, b) => {
                write!(f, "swap position {} with position {}", a, b)
            }
            Instruction::SwapLetters(a, b) => {
                write!(f, "swap letter {} with letter {}", a, b)
            }
            Instruction::RotateLeft(i) => {
                write!(f, "rotate left {} steps", i)
            }
            Instruction::RotateRight(i) => {
                write!(f, "rotate right {} steps", i)
            }
            Instruction::RotateBased(ch) => {
                write!(f, "rotate based on position of letter {}", ch)
            }
            Instruction::Reverse(a, b) => {
                write!(f, "reverse positions {} through {}", a, b)
            }
            Instruction::Move(a, b) => {
                write!(f, "move position {} to position {}", a, b)
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const SWAP_POSITIONS: &str = "swap position ";
        const SWAP_LETTERS: &str = "swap letter ";
        const ROTATE_LEFT: &str = "rotate left ";
        const ROTATE_RIGHT: &str = "rotate right ";
        const ROTATE_BASED: &str = "rotate based on position of letter ";
        const REVERSE: &str = "reverse positions ";
        const MOVE: &str = "move position ";

        if let Some(r) = s.strip_prefix(SWAP_POSITIONS) {
            let (start, end) = get_start_and_end(r)?;
            Ok(Instruction::SwapPositions(start.parse()?, end.parse()?))
        } else if let Some(r) = s.strip_prefix(SWAP_LETTERS) {
            let (start, end) = get_start_and_end(r)?;
            Ok(Instruction::SwapLetters(start.parse()?, end.parse()?))
        } else if let Some(r) = s.strip_prefix(ROTATE_LEFT) {
            let (start, _) = get_start_and_end(r)?;
            Ok(Instruction::RotateLeft(start.parse()?))
        } else if let Some(r) = s.strip_prefix(ROTATE_RIGHT) {
            let (start, _) = get_start_and_end(r)?;
            Ok(Instruction::RotateRight(start.parse()?))
        } else if let Some(r) = s.strip_prefix(ROTATE_BASED) {
            Ok(Instruction::RotateBased(r.parse()?))
        } else if let Some(r) = s.strip_prefix(REVERSE) {
            let (start, end) = get_start_and_end(r)?;
            Ok(Instruction::Reverse(start.parse()?, end.parse()?))
        } else if let Some(r) = s.strip_prefix(MOVE) {
            let (start, end) = get_start_and_end(r)?;
            Ok(Instruction::Move(start.parse()?, end.parse()?))
        } else {
            err_parse_error!("Invalid input: {}", s)
        }
    }
}

fn get_start_and_end(r: &str) -> Result<(&str, &str), ParseError> {
    if let (Some(idx1), Some(idx2)) = (r.find(' '), r.rfind(' ')) {
        Ok((&r[..idx1], &r[idx2 + 1..]))
    } else {
        err_parse_error!("Invalid input: {}", r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_swap_positions() {
        let mut chars = vec!['a', 'b', 'c', 'd', 'e'];
        Instruction::SwapPositions(1, 3).run(&mut chars);
        assert_eq!(chars, vec!['a', 'd', 'c', 'b', 'e']);
    }

    #[test]
    fn test_run_swap_letters() {
        let mut chars = vec!['a', 'b', 'c', 'd', 'e'];
        Instruction::SwapLetters('c', 'e').run(&mut chars);
        assert_eq!(chars, vec!['a', 'b', 'e', 'd', 'c']);
    }

    #[test]
    fn test_run_rotate_left_0() {
        let mut chars = vec!['a', 'b', 'c', 'd', 'e', 'f'];
        Instruction::RotateLeft(3).run(&mut chars);
        assert_eq!(chars, vec!['d', 'e', 'f', 'a', 'b', 'c']);
    }

    #[test]
    fn test_run_rotate_left_1() {
        let mut chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        Instruction::RotateLeft(3).run(&mut chars);
        assert_eq!(chars, vec!['d', 'e', 'f', 'g', 'a', 'b', 'c']);
    }

    #[test]
    fn test_run_rotate_left_2() {
        let mut chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        Instruction::RotateLeft(3).run(&mut chars);
        assert_eq!(chars, vec!['d', 'e', 'f', 'g', 'h', 'a', 'b', 'c']);
    }

    #[test]
    fn test_run_rotate_right_0() {
        let mut chars = vec!['a', 'b', 'c', 'd', 'e', 'f'];
        Instruction::RotateRight(3).run(&mut chars);
        assert_eq!(chars, vec!['d', 'e', 'f', 'a', 'b', 'c']);
    }

    #[test]
    fn test_run_rotate_right_1() {
        let mut chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        Instruction::RotateRight(3).run(&mut chars);
        assert_eq!(chars, vec!['e', 'f', 'g', 'a', 'b', 'c', 'd']);
    }

    #[test]
    fn test_run_rotate_right_2() {
        let mut chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        Instruction::RotateRight(3).run(&mut chars);
        assert_eq!(chars, vec!['f', 'g', 'h', 'a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    fn test_run_reverse_1() {
        let mut chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        Instruction::Reverse(2, 6).run(&mut chars);
        assert_eq!(chars, vec!['a', 'b', 'g', 'f', 'e', 'd', 'c', 'h']);
    }

    #[test]
    fn test_run_reverse_2() {
        let mut chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        Instruction::Reverse(1, 6).run(&mut chars);
        assert_eq!(chars, vec!['a', 'g', 'f', 'e', 'd', 'c', 'b', 'h']);
    }

    #[test]
    fn test_run_move() {
        let mut chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        Instruction::Move(1, 5).run(&mut chars);
        assert_eq!(chars, vec!['a', 'c', 'd', 'e', 'f', 'b', 'g', 'h']);
    }

    #[test]
    fn test_reverse_swap_positions() {
        let mut chars = vec!['a', 'd', 'c', 'b', 'e'];
        Instruction::SwapPositions(1, 3).reverse(&mut chars);
        assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    fn test_reverse_swap_letters() {
        let mut chars = vec!['a', 'b', 'e', 'd', 'c'];
        Instruction::SwapLetters('c', 'e').reverse(&mut chars);
        assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    fn test_reverse_rotate_left_0() {
        let mut chars = vec!['d', 'e', 'f', 'a', 'b', 'c'];
        Instruction::RotateLeft(3).reverse(&mut chars);
        assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e', 'f']);
    }

    #[test]
    fn test_reverse_rotate_left_1() {
        let mut chars = vec!['d', 'e', 'f', 'g', 'a', 'b', 'c'];
        Instruction::RotateLeft(3).reverse(&mut chars);
        assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);
    }

    #[test]
    fn test_reverse_rotate_left_2() {
        let mut chars = vec!['d', 'e', 'f', 'g', 'h', 'a', 'b', 'c'];
        Instruction::RotateLeft(3).reverse(&mut chars);
        assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']);
    }

    #[test]
    fn test_reverse_rotate_right_0() {
        let mut chars = vec!['d', 'e', 'f', 'a', 'b', 'c'];
        Instruction::RotateRight(3).reverse(&mut chars);
        assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e', 'f']);
    }

    #[test]
    fn test_reverse_rotate_right_1() {
        let mut chars = vec!['e', 'f', 'g', 'a', 'b', 'c', 'd'];
        Instruction::RotateRight(3).reverse(&mut chars);
        assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);
    }

    #[test]
    fn test_reverse_rotate_right_2() {
        let mut chars = vec!['f', 'g', 'h', 'a', 'b', 'c', 'd', 'e'];
        Instruction::RotateRight(3).reverse(&mut chars);
        assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']);
    }

    #[test]
    fn test_reverse_reverse_1() {
        let mut chars = vec!['a', 'b', 'g', 'f', 'e', 'd', 'c', 'h'];
        Instruction::Reverse(2, 6).reverse(&mut chars);
        assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']);
    }

    #[test]
    fn test_reverse_reverse_2() {
        let mut chars = vec!['a', 'g', 'f', 'e', 'd', 'c', 'b', 'h'];
        Instruction::Reverse(1, 6).reverse(&mut chars);
        assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']);
    }

    #[test]
    fn test_reverse_move() {
        let mut chars = vec!['a', 'c', 'd', 'e', 'f', 'b', 'g', 'h'];
        Instruction::Move(1, 5).reverse(&mut chars);
        assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']);
    }
}
