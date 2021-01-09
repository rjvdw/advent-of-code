use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let (valid_v1, valid_v2) = count_valid_passwords(&args[1]).or_exit_with(1);

    println!("[v1] There are {} valid passphrases.", valid_v1);
    println!("[v2] There are {} valid passphrases.", valid_v2);
}

fn count_valid_passwords(path: &str) -> Result<(usize, usize), io::Error> {
    let file = File::open(path)?;
    let mut count = (0, 0);
    for line in BufReader::new(file).lines() {
        let line = line?;
        if is_valid_v1(&line) {
            count.0 += 1;
        }
        if is_valid_v2(&line) {
            count.1 += 1;
        }
    }
    Ok(count)
}

fn is_valid_v1(passphrase: &str) -> bool {
    let mut seen: HashSet<&str> = HashSet::new();
    for word in passphrase.split_whitespace() {
        if seen.contains(word) {
            return false;
        }
        seen.insert(word);
    }
    true
}

fn is_valid_v2(passphrase: &str) -> bool {
    let mut seen: HashSet<String> = HashSet::new();
    for word in passphrase.split_whitespace() {
        let mut chars = word.chars().collect::<Vec<char>>();
        chars.sort_unstable();
        let mut word = String::new();
        for ch in chars {
            word.push(ch);
        }
        if seen.contains(&word) {
            return false;
        }
        seen.insert(word);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_v1() {
        assert!(is_valid_v1("aa bb cc dd ee"));
        assert!(!is_valid_v1("aa bb cc dd aa"));
        assert!(is_valid_v1("aa bb cc dd aaa"));
    }

    #[test]
    fn test_is_valid_v2() {
        assert!(is_valid_v2("abcde fghij"));
        assert!(!is_valid_v2("abcde xyz ecdab"));
        assert!(is_valid_v2("a ab abc abd abf abj"));
        assert!(is_valid_v2("iiii oiii ooii oooi oooo"));
        assert!(!is_valid_v2("oiii ioii iioi iiio"));
    }
}
