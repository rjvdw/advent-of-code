use rdcl_aoc_helpers::machine::hook::{HookResult, PreExecuteHook};
use rdcl_aoc_helpers::machine::instruction::MachineInstruction;
use rdcl_aoc_helpers::machine::output_receiver::OutputReceiver;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::machine::Machine;

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

impl PreExecuteHook for LoopDetector {
    fn run<I: MachineInstruction, R: MachineRegister, O: OutputReceiver>(
        &mut self,
        _machine: &mut Machine<I, R, O>,
        _instruction: &I,
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
