use std::cmp::Ordering;
use std::fmt;
use std::ops::Add;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

#[derive(Clone, Default)]
pub struct SnailNumber {
    value: Option<u8>,
    pair: Option<(Box<SnailNumber>, Box<SnailNumber>)>,
}

impl SnailNumber {
    /// To reduce a snailfish number, you must repeatedly do the first action in this list that
    /// applies to the snailfish number:
    ///
    /// * If any pair is nested inside four pairs, the leftmost such pair explodes.
    /// * If any regular number is 10 or greater, the leftmost such regular number splits.
    ///
    /// Once no action in the above list applies, the snailfish number is reduced.
    ///
    /// During reduction, at most one action applies, after which the process returns to the top of
    /// the list of actions. For example, if split produces a pair that meets the explode criteria,
    /// that pair explodes before other splits occur.
    fn reduce(&mut self) -> bool {
        self.explode(0).0 || self.split()
    }

    /// To explode a pair, the pair's left value is added to the first regular number to the left of
    /// the exploding pair (if any), and the pair's right value is added to the first regular number
    /// to the right of the exploding pair (if any). Exploding pairs will always consist of two
    /// regular numbers. Then, the entire exploding pair is replaced with the regular number 0.
    fn explode(&mut self, depth: usize) -> (bool, Option<u8>, Option<u8>) {
        if let Some((_, _)) = &self.pair {
            match depth.cmp(&3) {
                Ordering::Less => self.explode_recurse(depth),
                Ordering::Equal => self.explode_base(),
                Ordering::Greater => panic!("Recursion went to deep: {}", depth),
            }
        } else {
            (false, None, None)
        }
    }

    fn explode_recurse(&mut self, depth: usize) -> (bool, Option<u8>, Option<u8>) {
        let (left, right) = &mut self.pair.as_mut().unwrap();

        if let (true, value_left, value_right) = left.explode(depth + 1) {
            if let Some(vr) = value_right {
                if let Some(r) = &mut right.value {
                    *r += vr;
                } else {
                    right.add_to_left(vr);
                }

                return (true, value_left, None);
            }

            (true, value_left, value_right)
        } else if let (true, value_left, value_right) = right.explode(depth + 1) {
            if let Some(vl) = value_left {
                if let Some(l) = &mut left.value {
                    *l += vl;
                } else {
                    left.add_to_right(vl);
                }

                return (true, None, value_right);
            }

            (true, value_left, value_right)
        } else {
            (false, None, None)
        }
    }

    fn explode_base(&mut self) -> (bool, Option<u8>, Option<u8>) {
        let (left, right) = &mut self.pair.as_mut().unwrap();

        if let Some((left_value, right_value)) = left.explode_pair() {
            if let Some(rv) = &mut right.value {
                *rv += right_value;
            } else {
                right.add_to_left(right_value);
            }
            *left = Box::new(SnailNumber {
                value: Some(0),
                pair: None,
            });
            (true, Some(left_value), None)
        } else if let Some((left_value, right_value)) = right.explode_pair() {
            if let Some(lv) = &mut left.value {
                *lv += left_value;
            } else {
                left.add_to_right(left_value);
            }
            *right = Box::new(SnailNumber {
                value: Some(0),
                pair: None,
            });
            (true, None, Some(right_value))
        } else {
            (false, None, None)
        }
    }

    /// Splits up a `SnailNumber::Pair` into a `(u8, u8)`.
    fn explode_pair(&self) -> Option<(u8, u8)> {
        if let Some((left, right)) = &self.pair {
            if let Some(left_value) = left.value {
                if let Some(right_value) = right.value {
                    return Some((left_value, right_value));
                }
            }
            panic!("The nesting is going to deep ({:?}).", self);
        } else {
            None
        }
    }

    /// Finds the left-most `SnailNumber::Regular`, and adds `v` to its value.
    fn add_to_left(&mut self, value: u8) {
        if let Some(v) = &mut self.value {
            *v += value;
        } else {
            self.pair.as_mut().unwrap().0.add_to_left(value);
        }
    }

    /// Finds the right-most `SnailNumber::Regular`, and adds `v` to its value.
    fn add_to_right(&mut self, value: u8) {
        if let Some(v) = &mut self.value {
            *v += value;
        } else {
            self.pair.as_mut().unwrap().1.add_to_right(value);
        }
    }

    /// To split a regular number, replace it with a pair; the left element of the pair should be
    /// the regular number divided by two and rounded down, while the right element of the pair
    /// should be the regular number divided by two and rounded up. For example, 10 becomes \[5,5\],
    /// 11 becomes \[5,6\], 12 becomes \[6,6\], and so on.
    fn split(&mut self) -> bool {
        if let Some(v) = self.value {
            if v > 9 {
                let left = SnailNumber {
                    value: Some(v / 2),
                    pair: None,
                };
                let right = SnailNumber {
                    value: Some((v + 1) / 2),
                    pair: None,
                };
                self.pair = Some((Box::new(left), Box::new(right)));
                self.value = None;

                true
            } else {
                false
            }
        } else {
            let (left, right) = &mut self.pair.as_mut().unwrap();
            left.split() || right.split()
        }
    }

    /// The magnitude of a pair is 3 times the magnitude of its left element plus 2 times the
    /// magnitude of its right element. The magnitude of a regular number is just that number.
    pub fn magnitude(&self) -> u64 {
        if let Some((a, b)) = &self.pair {
            3 * a.magnitude() + 2 * b.magnitude()
        } else {
            self.value.unwrap() as u64
        }
    }
}

impl Add for SnailNumber {
    type Output = SnailNumber;

    /// To add two snailfish numbers, form a pair from the left and right parameters of the addition
    /// operator. For example, \[1,2\] + \[\[3,4\],5\] becomes \[\[1,2\],\[\[3,4\],5\]\].
    ///
    /// There's only one problem: snailfish numbers must always be reduced, and the process of
    /// adding two snailfish numbers can result in snailfish numbers that need to be reduced.
    ///
    /// Once no reduce actions apply, the snailfish number that remains is the actual result of the
    /// addition operation.
    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = SnailNumber {
            value: None,
            pair: Some((Box::new(self), Box::new(rhs))),
        };
        while sum.reduce() {}
        sum
    }
}

impl fmt::Debug for SnailNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((a, b)) = &self.pair {
            write!(f, "[{:?},{:?}]", a, b)
        } else {
            write!(f, "{}", self.value.unwrap())
        }
    }
}

impl FromStr for SnailNumber {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('[') {
            return Ok(SnailNumber {
                value: Some(s.parse()?),
                pair: None,
            });
        }

        let mut state = ParsingState {
            pair: (Default::default(), Default::default()),
            parsing: ParsingBranch::Left,
            stack: vec![],
        };

        for (pos, ch) in s.chars().into_iter().enumerate() {
            match ch {
                '[' => state.push(),
                ']' => state.pop(s, pos, ch)?,
                ',' => {
                    if let ParsingBranch::Left = state.parsing {
                        return Err(parse_error!("Illegal character {} @ {}: {}", ch, pos, s));
                    }
                }
                _ => state.parse_number(ch),
            }
        }

        if !state.is_done() {
            Err(parse_error!("Incomplete input string: {}", s))
        } else {
            Ok(state.pair.0)
        }
    }
}

struct ParsingState {
    pair: (SnailNumber, SnailNumber),
    parsing: ParsingBranch,
    stack: Vec<(ParsingBranch, (SnailNumber, SnailNumber))>,
}

impl ParsingState {
    fn push(&mut self) {
        let mut next_parsing = ParsingBranch::Left;
        let mut next_pair = (SnailNumber::default(), SnailNumber::default());
        std::mem::swap(&mut next_parsing, &mut self.parsing);
        std::mem::swap(&mut next_pair, &mut self.pair);
        self.stack.push((next_parsing, next_pair));
    }

    fn pop(&mut self, s: &str, i: usize, c: char) -> Result<(), ParseError> {
        if let ParsingBranch::Left = self.parsing {
            return Err(parse_error!("Illegal character {} @ {}: {}", c, i, s));
        }

        if let Some((parsing, mut pair)) = self.stack.pop() {
            std::mem::swap(&mut pair, &mut self.pair);
            let number = SnailNumber {
                value: None,
                pair: Some((Box::new(pair.0), Box::new(pair.1))),
            };
            match parsing {
                ParsingBranch::Left => {
                    self.pair.0 = number;
                    self.parsing = ParsingBranch::Right;
                }
                ParsingBranch::Right => {
                    self.pair.1 = number;
                }
            }

            Ok(())
        } else {
            Err(parse_error!("Illegal character {} @ {}: {}", c, i, s))
        }
    }

    fn parse_number(&mut self, c: char) {
        let number = SnailNumber {
            value: Some(c as u8 - b'0'),
            pair: None,
        };
        match self.parsing {
            ParsingBranch::Left => {
                self.pair.0 = number;
                self.parsing = ParsingBranch::Right;
            }
            ParsingBranch::Right => {
                self.pair.1 = number;
            }
        }
    }

    fn is_done(&self) -> bool {
        self.stack.is_empty()
    }
}

enum ParsingBranch {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explode() {
        let mut snail_number = sn("[[[[[9,8],1],2],3],4]");
        assert_eq!(snail_number.explode(0), (true, Some(9), None));
        assert_eq!(format!("{:?}", snail_number), "[[[[0,9],2],3],4]");

        let mut snail_number = sn("[7,[6,[5,[4,[3,2]]]]]");
        assert_eq!(snail_number.explode(0), (true, None, Some(2)));
        assert_eq!(format!("{:?}", snail_number), "[7,[6,[5,[7,0]]]]");

        let mut snail_number = sn("[[6,[5,[4,[3,2]]]],1]");
        assert_eq!(snail_number.explode(0), (true, None, None));
        assert_eq!(format!("{:?}", snail_number), "[[6,[5,[7,0]]],3]");

        let mut snail_number = sn("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        assert_eq!(snail_number.explode(0), (true, None, None));
        assert_eq!(
            format!("{:?}", snail_number),
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
        );

        let mut snail_number = sn("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        assert_eq!(snail_number.explode(0), (true, None, Some(2)));
        assert_eq!(
            format!("{:?}", snail_number),
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
        );

        let mut snail_number = sn("[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]]");
        assert_eq!(snail_number.explode(0), (true, Some(1), None));
        assert_eq!(
            format!("{:?}", snail_number),
            "[[[[0,[3,2]],[3,3]],[4,4]],[5,5]]"
        );
        assert_eq!(snail_number.explode(0), (true, None, None));
        assert_eq!(
            format!("{:?}", snail_number),
            "[[[[3,0],[5,3]],[4,4]],[5,5]]"
        );
    }

    #[test]
    fn test_add() {
        let a = sn("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let b = sn("[1,1]");
        assert_eq!(format!("{:?}", a + b), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

        let mut sum = sn("[1,1]") + sn("[2,2]") + sn("[3,3]") + sn("[4,4]");
        assert_eq!(format!("{:?}", sum), "[[[[1,1],[2,2]],[3,3]],[4,4]]");

        sum = sum + sn("[5,5]");
        assert_eq!(format!("{:?}", sum), "[[[[3,0],[5,3]],[4,4]],[5,5]]");

        sum = sum + sn("[6,6]");
        assert_eq!(format!("{:?}", sum), "[[[[5,0],[7,4]],[5,5]],[6,6]]");
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(sn("[[1,2],[[3,4],5]]").magnitude(), 143);
        assert_eq!(sn("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(), 1384);
        assert_eq!(sn("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(), 445);
        assert_eq!(sn("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(), 791);
        assert_eq!(sn("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(), 1137);
        assert_eq!(
            sn("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(),
            3488
        );
    }

    fn sn(s: &str) -> SnailNumber {
        s.parse().unwrap()
    }
}
