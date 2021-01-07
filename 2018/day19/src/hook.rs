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
        if self.1 {
            // todo: These optimizations are based on my input. YMMV.
            if idx == 1 {
                let f = machine.register.read('f');
                machine.register.write('a', find_sum_of_divisors(f));
                machine.register.write('c', 324);
            } else if idx == 17 {
                let a = machine.register.read('a');
                let d = machine.register.read('d');
                let f = machine.register.read('f');

                let new_d = 22 * d + 181;
                let new_f = 209 * (f + 2) * (f + 2) + new_d;

                machine.register.write('c', 25 + a);
                machine.register.write('d', new_d);
                machine.register.write('f', new_f);
            } else if idx == 27 {
                machine.register.write('a', 0);
                machine.register.write('c', 0);
                machine.register.write('d', 10550400);
                machine.register.increment('f', 10550400);
            } else {
                instruction.execute(&mut machine.register, &mut machine.output_receiver);
            }
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
