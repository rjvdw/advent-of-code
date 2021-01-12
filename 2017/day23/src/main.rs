use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;
use rdcl_aoc_helpers::machine::hook::{HookResult, PreExecuteHook};
use rdcl_aoc_helpers::machine::output_receiver::OutputReceiver;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::machine::Machine;

use crate::instruction::Instruction;

mod instruction;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let instructions: Vec<Instruction> = File::open(&args[1]).read_lines(1).collect();

    let mut machine = Machine::new_simple_machine(&instructions);
    let mut hook = MulCountHook { mul_count: 0 };
    machine.run(&mut hook);
    println!("The mul instruction is invoked {} times.", hook.mul_count);

    let mut machine = Machine::new_simple_machine(&instructions);
    machine.register.write('a', 1);
    machine.run(&mut OptimizingHook);
    println!("The value in register h is {}.", machine.register.read('h'));
}

struct MulCountHook {
    mul_count: usize,
}

impl PreExecuteHook<Instruction> for MulCountHook {
    fn run<R: MachineRegister, O: OutputReceiver<R>>(
        &mut self,
        _machine: &mut Machine<Instruction, R, O>,
        instruction: &Instruction,
        _idx: usize,
    ) -> HookResult {
        if matches!(instruction, Instruction::Mul(_, _)) {
            self.mul_count += 1;
        }
        HookResult::Proceed
    }
}

struct OptimizingHook;

impl PreExecuteHook<Instruction> for OptimizingHook {
    fn run<R: MachineRegister, O: OutputReceiver<R>>(
        &mut self,
        machine: &mut Machine<Instruction, R, O>,
        _instruction: &Instruction,
        idx: usize,
    ) -> HookResult {
        if idx == 8 {
            let b = machine.register.read('b');
            for d in 2.. {
                if d * d > b {
                    break;
                }
                if b % d == 0 {
                    machine.register.increment('h', 1);
                    break;
                }
            }
            HookResult::Goto(26)
        } else {
            HookResult::Proceed
        }
    }
}
