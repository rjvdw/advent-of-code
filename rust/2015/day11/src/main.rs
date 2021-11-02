use std::cmp::Ordering;
use std::process::exit;

use rdcl_aoc_helpers::args::get_args;

const LOWER: char = 'a';
const UPPER: char = 'z';
const ILLEGAL: [char; 3] = ['i', 'o', 'l'];
const PASSWORD_LENGTH: usize = 8;

fn main() {
    let args = get_args(&["<current password>"], 1);
    let mut password = as_vec(&args[1]);

    if password.len() != PASSWORD_LENGTH {
        eprintln!(
            "Password {} is not valid: Invalid length",
            as_string(&password)
        );
        exit(1);
    }

    loop {
        password = next(&password);
        if is_valid(&password) {
            break;
        }
    }

    println!("Santa's next password is: {}", as_string(&password));

    loop {
        password = next(&password);
        if is_valid(&password) {
            break;
        }
    }

    println!("And the password after that is: {}", as_string(&password));
}

fn as_vec(string: &str) -> Vec<char> {
    string.chars().collect()
}

fn as_string(chars: &[char]) -> String {
    chars.iter().collect()
}

fn next(password: &[char]) -> Vec<char> {
    match password.iter().rposition(|&ch| ch != UPPER) {
        Some(idx) => password
            .iter()
            .enumerate()
            .map(|(pos, &ch)| match pos.cmp(&idx) {
                Ordering::Less => ch,
                Ordering::Equal => {
                    let mut next_char = ((ch as u8) + 1) as char;
                    if ILLEGAL.contains(&next_char) {
                        next_char = ((next_char as u8) + 1) as char;
                    }
                    next_char
                }
                Ordering::Greater => LOWER,
            })
            .collect(),
        None => password.iter().map(|_| LOWER).collect(),
    }
}

fn is_valid(password: &[char]) -> bool {
    let mut has_triple = false;
    let mut has_doubles = false;
    let mut first_double = '_';
    for (idx, &letter) in password.iter().enumerate() {
        if ILLEGAL.contains(&letter) {
            return false;
        }

        if !has_triple && idx < PASSWORD_LENGTH - 2 {
            has_triple = ((password[idx + 1] as u8) == (letter as u8) + 1)
                && ((password[idx + 2] as u8) == (letter as u8) + 2)
        }

        if !has_doubles && idx < PASSWORD_LENGTH - 1 && password[idx + 1] == letter {
            if first_double == '_' {
                first_double = letter;
            } else if first_double != letter {
                has_doubles = true;
            }
        }
    }

    has_triple && has_doubles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_1() {
        assert_eq!(next(&as_vec("abc")), as_vec("abd"));
    }

    #[test]
    fn test_next_2() {
        assert_eq!(next(&as_vec("abz")), as_vec("aca"));
    }

    #[test]
    fn test_next_3() {
        assert_eq!(next(&as_vec("aza")), as_vec("azb"));
    }

    #[test]
    fn test_next_4() {
        assert_eq!(next(&as_vec("azz")), as_vec("baa"));
    }

    #[test]
    fn test_next_5() {
        assert_eq!(next(&as_vec("zzz")), as_vec("aaa"));
    }

    #[test]
    fn test_is_valid_1() {
        assert!(!is_valid(&as_vec("hijklmmn")));
    }

    #[test]
    fn test_is_valid_2() {
        assert!(!is_valid(&as_vec("abbceffg")));
    }

    #[test]
    fn test_is_valid_3() {
        assert!(!is_valid(&as_vec("abbcegjk")));
    }

    #[test]
    fn test_is_valid_4() {
        assert!(is_valid(&as_vec("abcdffaa")));
    }

    #[test]
    fn test_is_valid_5() {
        assert!(is_valid(&as_vec("ghjaabcc")));
    }
}
