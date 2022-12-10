//! Certain Advent of Code puzzles produce output that looks like this:
//!
//! ```text
//! #..#.####.#....#.....##..
//! #..#.#....#....#....#..#.
//! ####.###..#....#....#..#.
//! #..#.#....#....#....#..#.
//! #..#.#....#....#....#..#.
//! #..#.####.####.####..##..
//! ```
//!
//! This output represents text (this particular example reads: "HELLO").
//! This library provides a helper method to convert this type of output to a string.

#[cfg(test)]
/// The height of a character.
pub const CHAR_HEIGHT: usize = 6;
/// The width of a character. This includes an empty column on the right-hand side.
pub const CHAR_WIDTH: usize = 5;

/// The letter A.
///
/// ```text
/// .##..
/// #..#.
/// #..#.
/// ####.
/// #..#.
/// #..#.
/// ```
pub const A: &str = "011111100100100100011111000000";

/// The letter B.
///
/// ```text
/// ###..
/// #..#.
/// ###..
/// #..#.
/// #..#.
/// ###..
/// ```
pub const B: &str = "111111101001101001010110000000";

/// The letter C.
///
/// ```text
/// .##..
/// #..#.
/// #....
/// #....
/// #..#.
/// .##..
/// ````
pub const C: &str = "011110100001100001010010000000";

/// The letter D.
///
/// ```text
/// ###..
/// #..#.
/// #..#.
/// #..#.
/// #..#.
/// ###..
/// ```
pub const D: &str = "111111100001100001011110000000";

/// The letter E.
///
/// ```text
/// ####.
/// #....
/// ###..
/// #....
/// #....
/// ####.
/// ```
pub const E: &str = "111111101001101001100001000000";

/// The letter F.
///
/// ```text
/// ####.
/// #....
/// ###..
/// #....
/// #....
/// #....
/// ```
pub const F: &str = "111111101000101000100000000000";

/// The letter G.
///
/// ```text
/// .##..
/// #..#.
/// #....
/// #.##.
/// #..#.
/// .###.
/// ```
pub const G: &str = "011110100001100101010111000000";

/// The letter H.
///
/// ```text
/// #..#.
/// #..#.
/// ####.
/// #..#.
/// #..#.
/// #..#.
/// ```
pub const H: &str = "111111001000001000111111000000";

/// The letter I.
///
/// ```text
/// .###.
/// ..#..
/// ..#..
/// ..#..
/// ..#..
/// .###.
/// ```
pub const I: &str = "000000100001111111100001000000";

/// The letter J.
///
/// ```text
/// ..##.
/// ...#.
/// ...#.
/// ...#.
/// #..#.
/// .##..
/// ```
pub const J: &str = "000010000001100001111110000000";

/// The letter K.
///
/// ```text
/// #..#.
/// #.#..
/// ##...
/// #.#..
/// #.#..
/// #..#.
/// ```
pub const K: &str = "111111001000010110100001000000";

/// The letter L.
///
/// ```text
/// #....
/// #....
/// #....
/// #....
/// #....
/// ####.
/// ```
pub const L: &str = "111111000001000001000001000000";

/// The letter M does not seem to exist.
/// This is probably because it cannot be displayed in this small 4x6 pixel font.
pub const M: &str = "??????????????????????????????"; // not found yet

/// The letter N does not seem to exist.
pub const N: &str = "??????????????????????????????"; // not found yet

/// The letter O.
///
/// ```text
/// .##..
/// #..#.
/// #..#.
/// #..#.
/// #..#.
/// .##..
/// ```
pub const O: &str = "011110100001100001011110000000";

/// The letter P.
///
/// ```text
/// ###..
/// #..#.
/// #..#.
/// ###..
/// #....
/// #....
/// ```
pub const P: &str = "111111100100100100011000000000";

/// The letter Q does not seem to exist.
/// This might be because it cannot be displayed in this small 4x6 pixel font in a way where it
/// doesn't look too much like the letter O.
pub const Q: &str = "??????????????????????????????"; // not found yet

/// The letter R.
///
/// ```text
/// ###..
/// #..#.
/// #..#.
/// ###..
/// #.#..
/// #..#.
/// ```
pub const R: &str = "111111100100100110011001000000";

/// The letter S.
///
/// ```text
/// .###.
/// #....
/// #....
/// .##..
/// ...#.
/// ###..
/// ```
pub const S: &str = "011001100101100101100010000000";

/// The letter T.
///
/// ```text
/// .###.
/// ..#..
/// ..#..
/// ..#..
/// ..#..
/// ..#..
/// ```
pub const T: &str = "000000100000111111100000000000";

/// The letter U.
///
/// ```text
/// #..#.
/// #..#.
/// #..#.
/// #..#.
/// #..#.
/// .##..
/// ```
pub const U: &str = "111110000001000001111110000000";

/// The letter V does not seem to exist.
/// This is probably because it would look too much like the letter U.
pub const V: &str = "??????????????????????????????";

/// The letter W does not seem to exist.
/// This is probably because it cannot be displayed in this small 4x6 pixel font.
pub const W: &str = "??????????????????????????????"; // not found yet

/// The letter X does not seem to exist.
pub const X: &str = "??????????????????????????????"; // not found yet

/// The letter Y.
///
/// ```text
/// #...#
/// #...#
/// .#.#.
/// ..#..
/// ..#..
/// ..#..
/// ```
///
/// Notice that it uses the fifth column as well,
pub const Y: &str = "110000001000000111001000110000";

/// The letter Z.
///
/// ```text
/// ####.
/// ...#.
/// ..#..
/// .#...
/// #....
/// ####.
/// ```
pub const Z: &str = "100011100101101001110001000000";

/// An empty space.
pub const SPACE: &str = "000000000000000000000000000000";

/// Tries to match an input to a character.
///
/// * `spec` - A string representation of a 5x6 portion of the display, serialized by column.
///            Use `1` to indicate a pixel that is "on" and `0` to indicate a pixel that is "off".
///            The string should be exactly 5x6 = 30 characters long.
pub fn match_char(spec: &str) -> char {
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
    fn test_ocr() {
        // A    B    C    D    E    F    G    H    I    J    K    L    M    space
        let input = "\
          .##..###...##..###..####.####..##..#..#..###...##.#..#.#....####......\n\
          #..#.#..#.#..#.#..#.#....#....#..#.#..#...#.....#.#.#..#....####......\n\
          #..#.###..#....#..#.###..###..#....####...#.....#.##...#....####......\n\
          ####.#..#.#....#..#.#....#....#.##.#..#...#.....#.#.#..#....####......\n\
          #..#.#..#.#..#.#..#.#....#....#..#.#..#...#..#..#.#.#..#....####......\n\
          #..#.###...##..###..####.#.....###.#..#..###..##..#..#.####.####......\n\
        ";

        assert_eq!(ocr(input), "ABCDEFGHIJKL? ".to_string());

        // N    O    P    Q    R    S    T    U    V    W    X    Y    Z
        let input = "\
          ####..##..###..####.###...###..###.#..#.####.####.####.#...#####.\n\
          ####.#..#.#..#.####.#..#.#......#..#..#.####.####.####.#...#...#.\n\
          ####.#..#.#..#.####.#..#.#......#..#..#.####.####.####..#.#...#..\n\
          ####.#..#.###..####.###...##....#..#..#.####.####.####...#...#...\n\
          ####.#..#.#....####.#.#.....#...#..#..#.####.####.####...#..#....\n\
          ####..##..#....####.#..#.###....#...##..####.####.####...#..####.\n\
        ";

        assert_eq!(ocr(input), "?OP?RSTU???YZ".to_string());
    }
}
