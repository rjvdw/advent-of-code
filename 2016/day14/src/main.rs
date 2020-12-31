use std::collections::VecDeque;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

const K: usize = 1000;
const STRETCH_FACTOR: usize = 2016;

fn main() {
    let args = get_args(&["<salt>", "<n>"], 1);
    let salt = &args[1];
    let n = args[2].parse::<usize>().or_exit_with(1);

    let idx = find_nth_key(salt, n, compute_hash);
    println!("Using salt '{}', index {} produces key #{}.", salt, idx, n);

    let idx = find_nth_key(salt, n, compute_stretched_hash);
    println!(
        "With stretched hashes, using salt '{}', index {} produces key #{}.",
        salt, idx, n
    );
}

fn find_nth_key(salt: &str, mut n: usize, compute: fn(&str, usize) -> String) -> usize {
    let mut next_k_hashes: VecDeque<String> = VecDeque::with_capacity(K);
    for i in 0..K {
        next_k_hashes.push_back(compute(salt, i));
    }

    let mut idx = 0;
    while n > 0 {
        let hash = next_k_hashes.pop_front().unwrap();
        next_k_hashes.push_back(compute(salt, idx + K));
        if is_valid(&hash, &next_k_hashes) {
            n -= 1;
        }
        idx += 1;
    }

    idx - 1
}

fn compute_hash(salt: &str, idx: usize) -> String {
    let input = format!("{}{}", salt, idx);
    format!("{:x}", md5::compute(input))
}

fn compute_stretched_hash(salt: &str, idx: usize) -> String {
    let mut hash = compute_hash(salt, idx);
    for _ in 0..STRETCH_FACTOR {
        hash = format!("{:x}", md5::compute(hash));
    }
    hash
}

fn is_valid(hash: &str, next_k_hashes: &VecDeque<String>) -> bool {
    let mut p2 = ['_', '_'];
    let mut chars = hash.chars();
    let mut i = 0;
    p2[0] = chars.next().unwrap();
    p2[1] = chars.next().unwrap();
    let mut found_triple = false;
    for ch in chars {
        if ch == p2[0] && ch == p2[1] {
            found_triple = true;
            break;
        }
        p2[i] = ch;
        i = 1 - i;
    }
    let mut quint = String::new();
    for _ in 0..5 {
        quint.push(p2[0]);
    }
    found_triple && next_k_hashes.iter().any(|h| h.contains(&quint))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_nth_key_with_compute_hash() {
        assert_eq!(find_nth_key("abc", 64, compute_hash), 22728);
    }

    // FIXME: This test takes way too long, unless you run it with --release.
    // #[test]
    // fn test_find_nth_key_with_compute_stretched_hash() {
    //     assert_eq!(find_nth_key("abc", 64, compute_stretched_hash), 22551);
    // }
}
