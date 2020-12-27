use rdcl_aoc_helpers::args::get_args;

fn main() {
    let args = get_args(&["<secret key>"], 1);

    let (hash, advent_coin) = mine(&args[1], "00000", 0);
    println!("For number {}, the hash is {}", advent_coin, hash);

    let (hash, advent_coin) = mine(&args[1], "000000", advent_coin);
    println!("For number {}, the hash is {}", advent_coin, hash);
}

fn mine(secret_key: &str, lead: &str, start_from: usize) -> (String, usize) {
    let mut number = start_from;
    loop {
        number += 1;
        let input = format!("{}{}", secret_key, number);
        let hash = md5::compute(input);
        let formatted = format!("{:x}", hash);
        if formatted.starts_with(lead) {
            return (formatted, number);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine_1() {
        assert_eq!(mine("abcdef", "00000", 0).1, 609043);
    }

    #[test]
    fn test_mine_2() {
        assert_eq!(mine("pqrstuv", "00000", 0).1, 1048970);
    }
}
