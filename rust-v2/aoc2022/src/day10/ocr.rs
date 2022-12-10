#[cfg(test)]
const CHAR_HEIGHT: usize = 6;
const CHAR_WIDTH: usize = 5;

// .##..
// #..#.
// #..#.
// ####.
// #..#.
// #..#.
const A: &str = "011111100100100100011111000000";

// ###..
// #..#.
// ###..
// #..#.
// #..#.
// ###..
const B: &str = "111111101001101001010110000000";

// .##..
// #..#.
// #....
// #....
// #..#.
// .##..
const C: &str = "011110100001100001010010000000";

const D: &str = "??????????????????????????????"; // not found yet

// ####.
// #....
// ###..
// #....
// #....
// ####.
const E: &str = "111111101001101001100001000000";

// ####.
// #....
// ###..
// #....
// #....
// #....
const F: &str = "111111101000101000100000000000";

// .##..
// #..#.
// #....
// #.##.
// #..#.
// .###.
const G: &str = "011110100001100101010111000000";

// #..#.
// #..#.
// ####.
// #..#.
// #..#.
// #..#.
const H: &str = "111111001000001000111111000000";

const I: &str = "??????????????????????????????"; // not found yet

// ..##.
// ...#.
// ...#.
// ...#.
// #..#.
// .##..
const J: &str = "000010000001100001111110000000";

// #..#.
// #.#..
// ##...
// #.#..
// #.#..
// #..#.
const K: &str = "111111001000010110100001000000";

// #....
// #....
// #....
// #....
// #....
// ####.
const L: &str = "111111000001000001000001000000";

const M: &str = "??????????????????????????????"; // not found yet

const N: &str = "??????????????????????????????"; // not found yet

const O: &str = "??????????????????????????????"; // not found yet

// ###..
// #..#.
// #..#.
// ###..
// #....
// #....
const P: &str = "111111100100100100011000000000";

const Q: &str = "??????????????????????????????"; // not found yet

// ###..
// #..#.
// #..#.
// ###..
// #.#..
// #..#.
const R: &str = "111111100100100110011001000000";

const S: &str = "??????????????????????????????"; // not found yet

const T: &str = "??????????????????????????????"; // not found yet

// #..#.
// #..#.
// #..#.
// #..#.
// #..#.
// .##..
const U: &str = "111110000001000001111110000000";

const V: &str = "??????????????????????????????"; // not found yet

const W: &str = "??????????????????????????????"; // not found yet

const X: &str = "??????????????????????????????"; // not found yet

const Y: &str = "??????????????????????????????"; // not found yet

// ####.
// ...#.
// ..#..
// .#...
// #....
// ####.
const Z: &str = "100011100101101001110001000000";

const SPACE: &str = "000000000000000000000000000000";

/// Tries to match an input to a character.
fn match_char(spec: &str) -> char {
    match spec {
        v if v == A => 'A',
        v if v == B => 'B',
        v if v == C => 'C',
        v if v == D => 'D',
        v if v == E => 'E',
        v if v == F => 'F',
        v if v == G => 'G',
        v if v == H => 'H',
        v if v == I => 'I',
        v if v == J => 'J',
        v if v == K => 'K',
        v if v == L => 'L',
        v if v == M => 'M',
        v if v == N => 'N',
        v if v == O => 'O',
        v if v == P => 'P',
        v if v == Q => 'Q',
        v if v == R => 'R',
        v if v == S => 'S',
        v if v == T => 'T',
        v if v == U => 'U',
        v if v == V => 'V',
        v if v == W => 'W',
        v if v == X => 'X',
        v if v == Y => 'Y',
        v if v == Z => 'Z',
        v if v == SPACE => ' ',
        _ => '?',
    }
}

/// Tries to interpret the output as text.
///
/// Assumptions:
/// - Each character has a width of 5 pixels and a height of 6 pixels.
/// - The display only contains a single line of text.
/// - Each row in the display has the same width.
pub fn ocr(display: &str) -> String {
    let lines = display
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    let mut current = String::new();
    let mut text = String::new();

    for i in 0..lines[0].len() {
        for line in &lines {
            current.push(if line[i] { '1' } else { '0' });
        }

        if i % CHAR_WIDTH == CHAR_WIDTH - 1 {
            if !current.is_empty() {
                text.push(match_char(&current));
            }
            current.clear();
        }
    }

    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_length() {
        let expected = CHAR_WIDTH * CHAR_HEIGHT;

        assert_eq!(A.len(), expected);
        assert_eq!(B.len(), expected);
        assert_eq!(C.len(), expected);
        assert_eq!(D.len(), expected);
        assert_eq!(E.len(), expected);
        assert_eq!(F.len(), expected);
        assert_eq!(G.len(), expected);
        assert_eq!(H.len(), expected);
        assert_eq!(I.len(), expected);
        assert_eq!(J.len(), expected);
        assert_eq!(K.len(), expected);
        assert_eq!(L.len(), expected);
        assert_eq!(M.len(), expected);
        assert_eq!(N.len(), expected);
        assert_eq!(O.len(), expected);
        assert_eq!(P.len(), expected);
        assert_eq!(Q.len(), expected);
        assert_eq!(R.len(), expected);
        assert_eq!(S.len(), expected);
        assert_eq!(T.len(), expected);
        assert_eq!(U.len(), expected);
        assert_eq!(V.len(), expected);
        assert_eq!(W.len(), expected);
        assert_eq!(X.len(), expected);
        assert_eq!(Y.len(), expected);
        assert_eq!(Z.len(), expected);
        assert_eq!(SPACE.len(), expected);
    }

    #[test]
    fn test_ocr_1() {
        let input = "\
          .##..###...##..###..####.####..##..#..#.####...##.#..#.#....####......\n\
          #..#.#..#.#..#.#..#.#....#....#..#.#..#..#......#.#.#..#....####......\n\
          #..#.###..#....#..#.###..###..#....####..#......#.##...#....####......\n\
          ####.#..#.#....#..#.#....#....#.##.#..#..#......#.#.#..#....####......\n\
          #..#.#..#.#..#.#..#.#....#....#..#.#..#..#...#..#.#.#..#....####......\n\
          #..#.###...##..###..####.#.....###.#..#.####..##..#..#.####.####......\n\
        ";

        assert_eq!(ocr(input), "ABC?EFGH?JKL? ".to_string());
        let input = "\
          ####..##..###...##..###..####.####.#..#.####.####.####.####.####.\n\
          ####.#..#.#..#.#..#.#..#.####..#...#..#.####.####.####.####....#.\n\
          ####.#..#.#..#.#..#.#..#.####..#...#..#.####.####.####.####...#..\n\
          ####.#..#.###..#..#.###..####..#...#..#.####.####.####.####..#...\n\
          ####.#..#.#....#..#.#.#..####..#...#..#.####.####.####.####.#....\n\
          ####..##..#.....###.#..#.####..#....##..####.####.####.####.####.\n\
        ";

        assert_eq!(ocr(input), "??P?R??U????Z".to_string());
    }
}
