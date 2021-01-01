use std::collections::{HashMap, HashSet};
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use shared::instruction::{Instruction, Value};
use shared::output_receiver::OutputReceiver;
use shared::program::execute;

fn main() {
    let args = get_args(&["<input file>", "<use alternate solution? yes/no>"], 1);
    let instructions = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Instruction>>();

    if args[2] == "no" {
        let mut a = 0;
        let signal;
        loop {
            let mut registers = HashMap::new();
            registers.insert('a', a);
            registers.insert('b', 0);
            registers.insert('c', 0);
            registers.insert('d', 0);
            let mut antenna = Antenna::new();
            execute(&instructions, &mut registers, &mut antenna);

            if antenna.has_correct_signal() {
                signal = antenna.signal;
                break;
            } else {
                a += 1;
            }
        }
        println!("For a={}, the antenna will repeat {}...", a, signal);
    } else {
        match alternate_solution(&instructions) {
            Some(a) => println!("For a={}, the antenna will repeat 0,1,0,1,...", a),
            None => eprintln!("Could not find a value for a for which the antenna will repeat 0,1,0,1,... Maybe you have better luck if you do not use the alternate solution."),
        }
    }
}

fn alternate_solution(instructions: &[Instruction]) -> Option<i32> {
    // This is an alternate solution. It might only work for my specific input.
    let i1 = if let Instruction::Copy(Value::Raw(v), _) = instructions[1] {
        v
    } else {
        return None;
    };

    let i2 = if let Instruction::Copy(Value::Raw(v), _) = instructions[2] {
        v
    } else {
        return None;
    };

    let threshold = i1 * i2;
    let mut a = 0;
    let mut t = 1;
    while a < threshold {
        a = a * 2 + t;
        t = 1 - t;
    }

    Some(a - threshold)
}

struct Antenna {
    seen: HashSet<String>,
    signal: String,
}

impl Antenna {
    fn new() -> Antenna {
        Antenna {
            seen: HashSet::new(),
            signal: String::new(),
        }
    }

    fn has_correct_signal(&self) -> bool {
        let correct = ['0', ',', '1', ','];
        for (idx, ch) in self.signal.chars().enumerate() {
            if ch != correct[idx % correct.len()] {
                return false;
            }
        }
        true
    }

    fn hash_key(&self, registers: &HashMap<char, i32>) -> String {
        let mut keys = registers.keys().copied().collect::<Vec<char>>();
        keys.sort_unstable();
        let mut hash_key = String::new();
        for key in keys {
            if !hash_key.is_empty() {
                hash_key.push(',');
            }
            hash_key.push_str(&format!("{}={}", key, registers.get(&key).unwrap_or(&0)));
        }
        hash_key
    }
}

impl OutputReceiver for Antenna {
    fn receive(&mut self, output: i32, registers: &HashMap<char, i32>) -> bool {
        let hash_key = self.hash_key(registers);
        if self.seen.contains(&hash_key) {
            true
        } else {
            if !self.signal.is_empty() {
                self.signal.push(',');
            }
            self.signal.push_str(&output.to_string());
            // println!("{}, {}", output, hash_key);
            self.seen.insert(hash_key);
            false
        }
    }
}
