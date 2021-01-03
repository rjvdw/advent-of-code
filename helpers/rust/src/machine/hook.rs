use crate::machine::instruction::MachineInstruction;
use crate::machine::output_receiver::OutputReceiver;
use crate::machine::register::MachineRegister;
use crate::machine::Machine;
use std::marker::PhantomData;

/// Allows the hook to influence the rest of the execution of an instruction.
pub enum HookResult {
    /// Proceed normally.
    Proceed,

    /// Skip the instruction that was about to be executed.
    Skip,

    /// Go to a specific instruction.
    Goto(i64),

    /// Abort the entire program.
    Abort,
}

pub trait PreExecuteHook<I: MachineInstruction> {
    fn run<R: MachineRegister, O: OutputReceiver<R>>(
        &mut self,
        machine: &mut Machine<I, R, O>,
        instruction: &I,
        idx: usize,
    ) -> HookResult;
}

pub struct NoopHook<I: MachineInstruction> {
    _marker: PhantomData<I>,
}

impl<I: MachineInstruction> Default for NoopHook<I> {
    fn default() -> Self {
        NoopHook {
            _marker: Default::default(),
        }
    }
}

impl<I: MachineInstruction> PreExecuteHook<I> for NoopHook<I> {
    fn run<R: MachineRegister, O: OutputReceiver<R>>(
        &mut self,
        _machine: &mut Machine<I, R, O>,
        _instruction: &I,
        _idx: usize,
    ) -> HookResult {
        HookResult::Proceed
    }
}
