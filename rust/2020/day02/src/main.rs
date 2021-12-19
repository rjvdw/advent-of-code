extern crate rdcl_aoc_helpers;

use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use password::Password;

mod password;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let passwords = File::open(&args[1]).read_lines::<Password>(1);

    let mut counts = (0, 0);
    for password in passwords {
        if password.valid_according_to_old_job() {
            counts.0 += 1;
        }
        if password.valid_according_to_corporate_policy() {
            counts.1 += 1;
        }
    }

    println!(
        "Number of valid passwords according to old job: {}",
        counts.0
    );
    println!(
        "Number of valid passwords according to corporate policy: {}",
        counts.1
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_old_password_policy() {
        let pw1 = "1-3 a: abcde".parse::<Password>().unwrap();
        let pw2 = "1-3 b: cdefg".parse::<Password>().unwrap();
        let pw3 = "2-9 c: ccccccccc".parse::<Password>().unwrap();

        assert!(pw1.valid_according_to_old_job());
        assert!(!pw2.valid_according_to_old_job());
        assert!(pw3.valid_according_to_old_job());
    }

    #[test]
    fn test_corporate_password_policy() {
        let pw1 = "1-3 a: abcde".parse::<Password>().unwrap();
        let pw2 = "1-3 b: cdefg".parse::<Password>().unwrap();
        let pw3 = "2-9 c: ccccccccc".parse::<Password>().unwrap();

        assert!(pw1.valid_according_to_corporate_policy());
        assert!(!pw2.valid_according_to_corporate_policy());
        assert!(!pw3.valid_according_to_corporate_policy());
    }
}
