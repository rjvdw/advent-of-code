use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

fn main() {
    let args = get_args(
        &["<initial state>", "<first disk size>", "<second disk size>"],
        1,
    );
    let initial_state = &args[1];
    let first_disk_size = args[2].parse::<usize>().or_exit_with(1);
    let second_disk_size = args[3].parse::<usize>().or_exit_with(1);

    let checksum = fill_disk(initial_state, first_disk_size);
    println!("The checksum for the first disk is: {}", checksum);

    let checksum = fill_disk(initial_state, second_disk_size);
    println!("The checksum for the second disk is: {}", checksum);
}

fn fill_disk(initial_state: &str, desired_len: usize) -> String {
    let mut state = initial_state.to_string();
    while state.len() < desired_len {
        state = grow(&state, desired_len);
    }
    checksum(&state)
}

fn grow(input: &str, upper_bound: usize) -> String {
    let mut output = input.to_string();
    output.push('0');
    for ch in input.chars().rev() {
        match ch {
            '0' => output.push('1'),
            '1' => output.push('0'),
            _ => panic!("invalid input"),
        }
        if output.len() == upper_bound {
            break;
        }
    }
    output
}

fn checksum(input: &str) -> String {
    let mut checksum = input.to_string();
    while checksum.len() % 2 == 0 {
        let mut reduced = String::new();
        let mut chars = checksum.chars();
        while let (Some(ch1), Some(ch2)) = (chars.next(), chars.next()) {
            reduced.push(if ch1 == ch2 { '1' } else { '0' });
        }
        checksum = reduced
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grow() {
        assert_eq!(grow("1", usize::MAX), "100");
        assert_eq!(grow("0", usize::MAX), "001");
        assert_eq!(grow("11111", usize::MAX), "11111000000");
        assert_eq!(
            grow("111100001010", usize::MAX),
            "1111000010100101011110000"
        );
        assert_eq!(grow("111100001010", 20), "11110000101001010111");
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum("1"), "1");
        assert_eq!(checksum("11"), "1");
        assert_eq!(checksum("00"), "1");
        assert_eq!(checksum("10"), "0");
        assert_eq!(checksum("01"), "0");
        assert_eq!(checksum("110010110100"), "100");
    }

    #[test]
    fn test_fill_disk() {
        assert_eq!(fill_disk("10000", 20), "01100");
    }
}
