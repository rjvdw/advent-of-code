use rdcl_aoc_helpers::machine::hook::{HookResult, PreExecuteHook};
use rdcl_aoc_helpers::machine::instruction::MachineInstruction;
use rdcl_aoc_helpers::machine::output_receiver::OutputReceiver;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::machine::Machine;

use shared::device::instruction::{reg, Instruction};

pub struct Hook(char, bool);

impl Hook {
    pub fn new(instruction_pointer: i64, with_optimization: bool) -> Hook {
        Hook(reg(instruction_pointer), with_optimization)
    }
}

impl PreExecuteHook<Instruction> for Hook {
    fn run<R: MachineRegister, O: OutputReceiver<R>>(
        &mut self,
        machine: &mut Machine<Instruction, R, O>,
        instruction: &Instruction,
        idx: usize,
    ) -> HookResult {
        if self.1 && idx == 1 {
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
        machine.register.increment(self.0, 1);
        HookResult::Goto(machine.register.read(self.0))
    }
}

fn find_sum_of_divisors(n: i64) -> i64 {
    n + (1..=n / 2).filter(|v| n % v == 0).sum::<i64>()
}
