use rdcl_aoc_helpers::machine::hook::{HookResult, PreExecuteHook};
use rdcl_aoc_helpers::machine::instruction::MachineInstruction;
use rdcl_aoc_helpers::machine::output_receiver::OutputReceiver;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::machine::Machine;

use crate::device::instruction::{reg, Instruction};

pub struct Hook {
    register: char,
    day19_optimization_enabled: bool,
    debugging: bool,
}

impl Hook {
    pub fn new(instruction_pointer: i64) -> Hook {
        Hook {
            register: reg(instruction_pointer),
            day19_optimization_enabled: false,
            debugging: false,
        }
    }

    pub fn enable_day19_optimization(&mut self) {
        self.day19_optimization_enabled = true;
    }

    pub fn enable_debugging(&mut self) {
        self.debugging = true;
    }
}

impl PreExecuteHook<Instruction> for Hook {
    fn run<R: MachineRegister, O: OutputReceiver<R>>(
        &mut self,
        machine: &mut Machine<Instruction, R, O>,
        instruction: &Instruction,
        idx: usize,
    ) -> HookResult {
        if self.debugging {
            println!(
                "{:60} {:3}. {}",
                format!("{}", machine.register),
                idx,
                instruction
            );
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
        }
        machine.register.increment(self.register, 1);
        HookResult::Goto(machine.register.read(self.register))
    }
}

fn find_sum_of_divisors(n: i64) -> i64 {
    n + (1..=n / 2).filter(|v| n % v == 0).sum::<i64>()
}
