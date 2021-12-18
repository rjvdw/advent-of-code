use std::fmt;
use std::ops::Add;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

#[derive(Clone)]
pub enum SnailNumber {
    Regular(u8),
    Pair(Box<SnailNumber>, Box<SnailNumber>),
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
    pub fn reduce(&mut self) -> bool {
        let (exploded, _, _) = self.explode(0);
        if exploded {
            true
        } else {
            self.split()
        }
    }

    /// To explode a pair, the pair's left value is added to the first regular number to the left of
    /// the exploding pair (if any), and the pair's right value is added to the first regular number
    /// to the right of the exploding pair (if any). Exploding pairs will always consist of two
    /// regular numbers. Then, the entire exploding pair is replaced with the regular number 0.
    fn explode(&mut self, depth: usize) -> (bool, Option<u8>, Option<u8>) {
        match self {
            SnailNumber::Regular(_) => (false, None, None),
            SnailNumber::Pair(a, b) if depth < 3 => {
                let (exploded, left, right) = a.explode(depth + 1);
                if exploded {
                    return if let Some(vb) = right {
                        match **b {
                            SnailNumber::Regular(v) => *b = Box::new(SnailNumber::Regular(v + vb)),
                            _ => b.add_to_left(vb),
                        }
                        (true, left, None)
                    } else {
                        (true, left, right)
                    };
                }

                let (exploded, left, right) = b.explode(depth + 1);
                if exploded {
                    return if let Some(va) = left {
                        match **a {
                            SnailNumber::Regular(v) => *a = Box::new(SnailNumber::Regular(v + va)),
                            _ => a.add_to_right(va),
                        }
                        (true, None, right)
                    } else {
                        (true, left, right)
                    };
                }

                (false, None, None)
            }
            SnailNumber::Pair(a, b) if depth == 3 => {
                if let Some((va, vb)) = a.explode_pair().unwrap() {
                    *a = Box::new(SnailNumber::Regular(0));
                    match **b {
                        SnailNumber::Regular(v) => *b = Box::new(SnailNumber::Regular(v + vb)),
                        _ => b.add_to_left(vb),
                    }
                    (true, Some(va), None)
                } else if let Some((va, vb)) = b.explode_pair().unwrap() {
                    *b = Box::new(SnailNumber::Regular(0));
                    match **a {
                        SnailNumber::Regular(v) => *a = Box::new(SnailNumber::Regular(v + va)),
                        _ => a.add_to_right(va),
                    }
                    (true, None, Some(vb))
                } else {
                    (false, None, None)
                }
            }
            _ => panic!("Recursion went too deep."),
        }
    }

    /// Splits up a `SnailNumber::Pair` into a `(u8, u8)`.
    fn explode_pair(&self) -> Result<Option<(u8, u8)>, ()> {
        match self {
            SnailNumber::Regular(_) => Ok(None),
            SnailNumber::Pair(a, b) => {
                if let SnailNumber::Regular(va) = **a {
                    if let SnailNumber::Regular(vb) = **b {
                        return Ok(Some((va, vb)));
                    }
                }
                Err(())
            }
        }
    }

    /// Finds the left-most `SnailNumber::Regular`, and adds `v` to its value.
    fn add_to_left(&mut self, v: u8) {
        match self {
            SnailNumber::Regular(_) => panic!("Something went wrong."),
            SnailNumber::Pair(a, _) => {
                if let SnailNumber::Regular(va) = **a {
                    *a = Box::new(SnailNumber::Regular(v + va));
                } else {
                    a.add_to_left(v);
                }
            }
        }
    }

    /// Finds the right-most `SnailNumber::Regular`, and adds `v` to its value.
    fn add_to_right(&mut self, v: u8) {
        match self {
            SnailNumber::Regular(_) => panic!("Something went wrong."),
            SnailNumber::Pair(_, b) => {
                if let SnailNumber::Regular(vb) = **b {
                    *b = Box::new(SnailNumber::Regular(v + vb));
                } else {
                    b.add_to_right(v);
                }
            }
        }
    }

    /// To split a regular number, replace it with a pair; the left element of the pair should be
    /// the regular number divided by two and rounded down, while the right element of the pair
    /// should be the regular number divided by two and rounded up. For example, 10 becomes \[5,5\],
    /// 11 becomes \[5,6\], 12 becomes \[6,6\], and so on.
    fn split(&mut self) -> bool {
        match self {
            SnailNumber::Regular(_) => false,
            SnailNumber::Pair(a, b) => {
                if let SnailNumber::Regular(v) = **a {
                    if v > 9 {
                        let ra = SnailNumber::Regular(v / 2);
                        let rb = SnailNumber::Regular((v + 1) / 2);
                        *a = Box::new(SnailNumber::Pair(Box::new(ra), Box::new(rb)));
                        return true;
                    }
                }
                if a.split() {
                    return true;
                }
                if let SnailNumber::Regular(v) = **b {
                    if v > 9 {
                        let ra = SnailNumber::Regular(v / 2);
                        let rb = SnailNumber::Regular((v + 1) / 2);
                        *b = Box::new(SnailNumber::Pair(Box::new(ra), Box::new(rb)));
                        return true;
                    }
                }
                if b.split() {
                    return true;
                }
                false
            }
        }
    }

    /// The magnitude of a pair is 3 times the magnitude of its left element plus 2 times the
    /// magnitude of its right element. The magnitude of a regular number is just that number.
    pub fn magnitude(&self) -> u64 {
        match self {
            SnailNumber::Regular(v) => *v as u64,
            SnailNumber::Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
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
        let mut sum = SnailNumber::Pair(Box::new(self), Box::new(rhs));
        while sum.reduce() {}
        sum
    }
}

impl fmt::Debug for SnailNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SnailNumber::Regular(nr) => write!(f, "{}", nr),
            SnailNumber::Pair(a, b) => write!(f, "[{:?},{:?}]", a, b),
        }
    }
}

impl Default for SnailNumber {
    fn default() -> Self {
        SnailNumber::Regular(0)
    }
}

impl FromStr for SnailNumber {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('[') {
            return Ok(SnailNumber::Regular(s.parse()?));
        }
        let mut pair = (SnailNumber::default(), SnailNumber::default());
        let mut parsing = Parsing::Left;
        let mut stack = vec![];
        for (pos, ch) in s.chars().into_iter().enumerate() {
            match ch {
                '[' => {
                    stack.push((parsing, pair));
                    pair = (SnailNumber::default(), SnailNumber::default());
                    parsing = Parsing::Left;
                }
                ']' => {
                    if matches!(parsing, Parsing::Left) {
                        return Err(parse_error!("Illegal character {} @ {}: {}", ch, pos, s));
                    }

                    if let Some((parsing_prev, mut pair_prev)) = stack.pop() {
                        let number = SnailNumber::Pair(Box::new(pair.0), Box::new(pair.1));
                        match parsing_prev {
                            Parsing::Left => {
                                pair_prev.0 = number;
                                parsing = Parsing::Right;
                            }
                            Parsing::Right => pair_prev.1 = number,
                        }
                        pair = pair_prev;
                    } else {
                        return Err(parse_error!("Illegal character {} @ {}: {}", ch, pos, s));
                    }
                }
                ',' => {
                    if matches!(parsing, Parsing::Left) {
                        return Err(parse_error!("Illegal character {} @ {}: {}", ch, pos, s));
                    }
                }
                _ => {
                    let number = SnailNumber::Regular((ch as u8) - b'0');
                    match parsing {
                        Parsing::Left => {
                            pair.0 = number;
                            parsing = Parsing::Right
                        }
                        Parsing::Right => pair.1 = number,
                    }
                }
            }
        }

        if stack.is_empty() {
            Ok(pair.0)
        } else {
            Err(parse_error!("Incomplete input string: {}", s))
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Parsing {
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
