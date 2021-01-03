use std::collections::HashSet;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;
use rdcl_aoc_helpers::machine::instruction::Value;
use rdcl_aoc_helpers::machine::output_receiver::OutputReceiver;
use rdcl_aoc_helpers::machine::register::{HashMapRegister, MachineRegister};
use rdcl_aoc_helpers::machine::Machine;

use shared::instruction::Instruction;
use shared::program::Hook;

fn main() {
    let args = get_args(&["<input file>", "<use alternate solution? yes/no>"], 1);
    let instructions = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Instruction>>();

    if args[2] == "no" {
        let mut a = 0;
        let signal;
        loop {
            let mut register = HashMapRegister::new();
            register.write('a', a);
            register.write('b', 0);
            register.write('c', 0);
            register.write('d', 0);

            let antenna = Antenna::new();
            let mut machine = Machine::new_machine(&instructions, register, antenna);
            machine.run(&mut Hook);

            if machine.output_receiver.has_correct_signal() {
                signal = machine.output_receiver.signal;
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

fn alternate_solution(instructions: &[Instruction]) -> Option<i64> {
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

#[derive(Debug)]
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
}

impl<T: MachineRegister> OutputReceiver<T> for Antenna {
    fn receive(&mut self, output: i64, register: &T) -> bool {
        let hash_key = format!("{}", register);
        if self.seen.contains(&hash_key) {
            true
        } else {
            if !self.signal.is_empty() {
                self.signal.push(',');
            }
            self.signal.push_str(&output.to_string());
            // println!("{}, {}", output, register);
            self.seen.insert(hash_key);
            false
        }
    }
}
