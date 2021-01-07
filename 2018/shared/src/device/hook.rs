use std::collections::HashSet;

use rdcl_aoc_helpers::machine::hook::{HookResult, PreExecuteHook};
use rdcl_aoc_helpers::machine::instruction::MachineInstruction;
use rdcl_aoc_helpers::machine::output_receiver::OutputReceiver;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::machine::Machine;

use crate::device::instruction::{reg, Instruction};

pub struct Hook {
    register: char,
    day19_optimization_enabled: bool,
    day21_analysis_enabled: bool,
    day21_seen: HashSet<i64>,
    debugging: bool,
    log_points: HashSet<usize>,
    instruction_counter: usize,
}

impl Hook {
    pub fn new(instruction_pointer: i64) -> Hook {
        Hook {
            register: reg(instruction_pointer),
            day19_optimization_enabled: false,
            day21_analysis_enabled: false,
            day21_seen: HashSet::new(),
            debugging: false,
            log_points: HashSet::new(),
            instruction_counter: 0,
        }
    }

    pub fn get_instructions_count(&self) -> usize {
        self.instruction_counter
    }

    pub fn enable_day19_optimization(&mut self) {
        self.day19_optimization_enabled = true;
    }

    pub fn enable_day21_analysis(&mut self) {
        self.day21_analysis_enabled = true;
    }

    pub fn enable_debugging(&mut self) {
        self.debugging = true;
    }

    pub fn add_log_points(&mut self, instructions: &[usize]) {
        for instruction in instructions {
            self.log_points.insert(*instruction);
        }
    }
}

impl PreExecuteHook<Instruction> for Hook {
    fn run<R: MachineRegister, O: OutputReceiver<R>>(
        &mut self,
        machine: &mut Machine<Instruction, R, O>,
        instruction: &Instruction,
        idx: usize,
    ) -> HookResult {
        if self.debugging && (self.log_points.is_empty() || self.log_points.contains(&idx)) {
            println!(
                "{:60} {:3}. {}",
                format!("{}", machine.register),
                idx,
                instruction
            );
        }

        if self.day21_analysis_enabled && idx == 28 {
            let d = machine.register.read('d');
            let x = machine.register.read('x');
            if x == 0 {
                machine.register.write('x', d);
                println!(
                    "[Part 1] The value for a should be {} (instruction count: {}).",
                    d, self.instruction_counter
                );
            }
            if self.day21_seen.contains(&d) {
                println!(
                    "[Part 2] The value for a should be {} (instruction count: {}).",
                    machine.register.read('y'),
                    self.instruction_counter
                );
                return HookResult::Abort;
            } else {
                machine.register.write('y', d);
                self.day21_seen.insert(d);
            }
        }

        if self.day19_optimization_enabled && idx == 1 {
            let f = machine.register.read('f');

            // Strictly speaking, we don't need to set all these registers (just 'a' and 'c'), but I
            // think it's nice if the entire machine halts in the correct state.
            machine.register.write('a', find_sum_of_divisors(f));
            machine.register.write('b', f);
            machine.register.write('c', 15);
            machine.register.write('d', 1);
            machine.register.write('e', f + 1);
        } else {
            instruction.execute(&mut machine.register, &mut machine.output_receiver);
            self.instruction_counter += 1;
        }

        machine.register.increment(self.register, 1);
        HookResult::Goto(machine.register.read(self.register))
    }
}

fn find_sum_of_divisors(n: i64) -> i64 {
    n + (1..=n / 2).filter(|v| n % v == 0).sum::<i64>()
}
