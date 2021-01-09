use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};

fn main() {
    let args = get_args(&["<input file>"], 1);
    let memory_banks = parse_input(&args[1]).or_exit_with(1);

    match detect_infinite_loop(&memory_banks) {
        Some((steps, period)) => println!(
            "An infinite loop is detected after {} steps. It repeats with period {}.",
            steps, period
        ),
        None => println!("No infinite loop is detected."),
    }
}

fn parse_input(path: &str) -> Result<Vec<usize>, ParseError> {
    let mut result = Vec::new();
    let file = File::open(path)?;
    if let Some(Ok(line)) = BufReader::new(file).lines().next() {
        for nr in line.split_whitespace() {
            result.push(nr.parse()?);
        }
    }
    Ok(result)
}

fn detect_infinite_loop(memory_banks: &[usize]) -> Option<(usize, usize)> {
    let n = memory_banks.len();
    let mut memory_banks = memory_banks.to_vec();
    let mut seen: HashMap<String, usize> = HashMap::new();
    seen.insert(get_hash_key(&memory_banks), 0);
    let mut steps = 0;

    loop {
        steps += 1;

        let (mut idx, value) = memory_banks
            .iter()
            .enumerate()
            .max_by(|&a, &b| a.1.cmp(b.1).then_with(|| a.0.cmp(&b.0).reverse()))?;

        let mut value = *value;
        memory_banks[idx] = 0;

        while value > 0 {
            idx = (idx + 1) % n;
            memory_banks[idx] += 1;
            value -= 1;
        }

        let key = get_hash_key(&memory_banks);
        if let Some(s) = seen.get(&key) {
            return Some((steps, steps - *s));
        }
        seen.insert(key, steps);
    }
}

fn get_hash_key(memory_banks: &[usize]) -> String {
    let mut key = String::new();
    for &bank in memory_banks {
        if !key.is_empty() {
            key.push(',');
        }
        key.push_str(&bank.to_string());
    }
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_infinite_loop() {
        assert_eq!(detect_infinite_loop(&[0, 2, 7, 0]), Some((5, 4)));
    }
}
