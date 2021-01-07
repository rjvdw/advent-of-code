use rdcl_aoc_helpers::machine::hook::{HookResult, PreExecuteHook};
use rdcl_aoc_helpers::machine::instruction::MachineInstruction;
use rdcl_aoc_helpers::machine::output_receiver::OutputReceiver;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::machine::Machine;

use shared::device::instruction::{reg, Instruction};

pub struct Hook(char);

impl Hook {
    pub fn new(instruction_pointer: i64) -> Hook {
        Hook(reg(instruction_pointer))
    }
}

impl PreExecuteHook<Instruction> for Hook {
    fn run<R: MachineRegister, O: OutputReceiver<R>>(
        &mut self,
        machine: &mut Machine<Instruction, R, O>,
        instruction: &Instruction,
        _idx: usize,
    ) -> HookResult {
        instruction.execute(&mut machine.register, &mut machine.output_receiver);
        machine.register.increment(self.0, 1);
        HookResult::Goto(machine.register.read(self.0))
    }
}
