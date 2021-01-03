use crate::machine::hook::{HookResult, PreExecuteHook};
use crate::machine::instruction::MachineInstruction;
use crate::machine::output_receiver::{NoopOutputReceiver, OutputReceiver};
use crate::machine::register::{HashMapRegister, MachineRegister};
use std::convert::TryFrom;

/// Hooks allow you to influence how the program is executed.
pub mod hook;

/// A machine instruction.
pub mod instruction;

/// Whenever your machine produces output.
pub mod output_receiver;

/// The registers of the machine.
pub mod register;

/// A machine which can run instructions.
pub struct Machine<I: MachineInstruction, R: MachineRegister, O: OutputReceiver<R>> {
    instructions: Vec<I>,
    pub register: R,
    pub output_receiver: O,
    counter: i32,
}

impl<I: MachineInstruction> Machine<I, HashMapRegister, NoopOutputReceiver> {
    /// Constructs a new machine which uses a HashMapRegister and the NoopOutputReceiver.
    pub fn new_simple_machine(
        instructions: &[I],
    ) -> Machine<I, HashMapRegister, NoopOutputReceiver> {
        Machine {
            instructions: instructions.to_vec(),
            register: HashMapRegister::default(),
            output_receiver: NoopOutputReceiver,
            counter: 0,
        }
    }
}

impl<I, R, O> Machine<I, R, O>
where
    I: MachineInstruction,
    R: MachineRegister,
    O: OutputReceiver<R>,
{
    /// Constructs a new machine where you specify your own register and output receiver.
    pub fn new_machine(instructions: &[I], register: R, output_receiver: O) -> Machine<I, R, O> {
        Machine {
            instructions: instructions.to_vec(),
            register,
            output_receiver,
            counter: 0,
        }
    }

    /// Let the program run.
    pub fn run<H: PreExecuteHook<I>>(&mut self, pre_execute_hook: &mut H) {
        while let Some((idx, instruction)) = self.get_next_instruction() {
            match pre_execute_hook.run(self, &instruction, idx) {
                HookResult::Proceed => {
                    self.counter +=
                        instruction.execute(&mut self.register, &mut self.output_receiver);
                }
                HookResult::Skip => {
                    self.counter += 1;
                }
                HookResult::Goto(idx) => {
                    self.counter = idx;
                }
                HookResult::Abort => {
                    return;
                }
            }
        }
    }

    /// Returns the program counter.
    pub fn get_counter(&self) -> i32 {
        self.counter
    }

    /// Fetch the instruction at position `idx`, if any.
    pub fn get_instruction(&self, idx: i32) -> Option<(usize, I)> {
        usize::try_from(idx).ok().and_then(|idx| {
            self.instructions
                .get(idx)
                .map(|instruction| (idx, instruction.clone()))
        })
    }

    /// Replaces the instruction at position `idx`, if any.
    pub fn set_instruction(&mut self, idx: i32, new_instruction: &I) {
        usize::try_from(idx).ok().and_then(|idx| {
            self.instructions
                .get_mut(idx)
                .map(|instruction| *instruction = new_instruction.clone())
        });
    }

    /// Fetch the next instruction, if any.
    fn get_next_instruction(&self) -> Option<(usize, I)> {
        self.get_instruction(self.counter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ParseError;
    use crate::machine::hook::NoopHook;
    use crate::machine::instruction::ParsedMachineInstruction;

    #[test]
    fn test_machine() {
        let instructions = vec![MockInstruction; 3];
        let mut machine = Machine::new_simple_machine(&instructions);

        machine.run(&mut NoopHook::default());

        assert_eq!(machine.register.read('a'), 30);
    }

    #[derive(Clone)]
    struct MockInstruction;

    impl MachineInstruction for MockInstruction {
        fn execute<R: MachineRegister, O: OutputReceiver<R>>(
            &self,
            register: &mut R,
            _output_receiver: &mut O,
        ) -> i32 {
            register.increment('a', 10);
            1
        }

        fn from_parsed_machine_instruction(
            _parsed: &ParsedMachineInstruction,
        ) -> Result<Self, ParseError> {
            unimplemented!()
        }
    }
}
