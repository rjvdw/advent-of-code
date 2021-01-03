use crate::machine::instruction::MachineInstruction;
use crate::machine::output_receiver::OutputReceiver;
use crate::machine::register::MachineRegister;
use crate::machine::Machine;

/// Allows the hook to influence the rest of the execution of an instruction.
pub enum HookResult {
    /// Proceed normally.
    Proceed,

    /// Skip the instruction that was about to be executed.
    Skip,

    /// Abort the entire program.
    Abort,
}

pub trait PreExecuteHook {
    fn run<I: MachineInstruction, R: MachineRegister, O: OutputReceiver>(
        &mut self,
        machine: &mut Machine<I, R, O>,
        instruction: &I,
        idx: usize,
    ) -> HookResult;
}

pub struct NoopHook;

impl PreExecuteHook for NoopHook {
    fn run<I: MachineInstruction, R: MachineRegister, O: OutputReceiver>(
        &mut self,
        _machine: &mut Machine<I, R, O>,
        _instruction: &I,
        _idx: usize,
    ) -> HookResult {
        HookResult::Proceed
    }
}
