use std::collections::HashSet;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;
use rdcl_aoc_helpers::machine::output_receiver::OutputReceiver;
use rdcl_aoc_helpers::machine::register::{HashMapRegister, MachineRegister};
use rdcl_aoc_helpers::machine::Machine;

use shared::instruction::Instruction;
use shared::program::Hook;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let instructions = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Instruction>>();

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
