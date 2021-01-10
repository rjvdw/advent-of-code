use rdcl_aoc_helpers::machine::hook::{HookResult, PreExecuteHook};
use rdcl_aoc_helpers::machine::output_receiver::OutputReceiver;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::machine::Machine;

use crate::instruction::Instruction;

pub struct LoopDetector {
    seen: Vec<bool>,
    aborted: bool,
}

impl LoopDetector {
    pub fn new(size: usize) -> LoopDetector {
        LoopDetector {
            seen: vec![false; size],
            aborted: false,
        }
    }

    pub fn terminated_normally(&self) -> bool {
        !self.aborted
    }
}

impl PreExecuteHook<Instruction> for LoopDetector {
    fn run<R: MachineRegister, O: OutputReceiver<R>>(
        &mut self,
        _machine: &mut Machine<Instruction, R, O>,
        _instruction: &Instruction,
        idx: usize,
    ) -> HookResult {
        if self.seen[idx] {
            self.aborted = true;
            HookResult::Abort
        } else {
            self.seen[idx] = true;
            HookResult::Proceed
        }
    }
}
