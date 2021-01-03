use rdcl_aoc_helpers::machine::output_receiver::NoopOutputReceiver;
use rdcl_aoc_helpers::machine::register::HashMapRegister;
use rdcl_aoc_helpers::machine::Machine;

use crate::instruction::Instruction;

pub struct Mutator {
    instructions: Vec<Instruction>,
    idx: usize,
}

impl Mutator {
    pub fn new(instructions: &[Instruction]) -> Mutator {
        Mutator {
            instructions: instructions.to_vec(),
            idx: 0,
        }
    }
}

impl Iterator for Mutator {
    type Item = (
        usize,
        Machine<Instruction, HashMapRegister, NoopOutputReceiver>,
    );

    fn next(&mut self) -> Option<Self::Item> {
        let next_instruction_opt = self
            .instructions
            .iter()
            .enumerate()
            .skip(self.idx)
            .find(|(_, instruction)| instruction.is_jmp() || instruction.is_nop());

        if let Some((idx, instruction)) = next_instruction_opt {
            self.idx = idx + 1;
            let mut altered = self.instructions.to_vec();
            altered[idx] = match instruction {
                Instruction::Nop(value) => Instruction::Jmp(*value),
                Instruction::Jmp(value) => Instruction::Nop(*value),
                _ => unreachable!(),
            };

            Some((idx, Machine::new_simple_machine(&altered)))
        } else {
            None
        }
    }
}
