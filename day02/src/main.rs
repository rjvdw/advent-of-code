use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input file>", &args[0]);
        exit(1);
    }

    let path = &args[1];
    let values = read_input(path);

    println!("Number of valid passwords according to old job: {}", values.iter()
        .filter(|v| v.valid_according_to_old_job()).count());
    println!("Number of valid passwords according to corporate policy: {}", values.iter()
        .filter(|v| v.valid_according_to_corporate_policy()).count());
}

fn read_input(path: &String) -> Vec<InputRecord> {
    let file = File::open(path).expect("Cannot read input");
    let mut values = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.expect("Unable to read line");
        let line = line.parse::<InputRecord>().expect("Unable to parse value");
        values.push(line);
    }

    return values;
}

#[derive(Debug)]
struct InputRecord {
    idx1: usize,
    idx2: usize,
    character: char,
    password: String,
}

impl InputRecord {
    fn valid_according_to_old_job(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.character).count();

        count >= self.idx1 && count <= self.idx2
    }

    fn valid_according_to_corporate_policy(&self) -> bool {
        let c1 = self.password.chars().nth(self.idx1 - 1).filter(|&c| c == self.character);
        let c2 = self.password.chars().nth(self.idx2 - 1).filter(|&c| c == self.character);

        (c1.is_some() && c2.is_none()) || (c1.is_none() && c2.is_some())
    }
}

impl FromStr for InputRecord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p1 = s.chars().position(|c| c == '-').expect(format!("Invalid input string: {}", s).as_str());
        let p2 = p1 + s.chars().skip(p1).position(|c| c == ' ').expect(format!("Invalid input string: {}", s).as_str());
        let p3 = p2 + s.chars().skip(p2).position(|c| c == ':').expect(format!("Invalid input string: {}", s).as_str());

        let idx1 = s[..p1].parse::<usize>().expect(format!("Invalid lower bound: {}", s).as_str());
        let idx2 = s[p1+1..p2].parse::<usize>().expect(format!("Invalid upper bound: {}", s).as_str());
        let character = s.chars().nth(p2+1).expect(format!("Invalid character: {}", s).as_str());
        let password = s.chars().skip(p3 + 2).collect();

        Ok(InputRecord { idx1, idx2, character, password })
    }
}
