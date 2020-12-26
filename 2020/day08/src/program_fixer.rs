use crate::instruction::{Instruction, Operation};
use crate::ProgramEndState;

pub struct ProgramFixer<'a> {
    instructions: &'a [Instruction],
    run_program: fn(&[Instruction]) -> ProgramEndState,
    idx: usize,
}

impl ProgramFixer<'_> {
    pub fn new(
        instructions: &[Instruction],
        solve: fn(&[Instruction]) -> ProgramEndState,
    ) -> ProgramFixer {
        ProgramFixer {
            instructions,
            run_program: solve,
            idx: 0,
        }
    }
}

impl Iterator for ProgramFixer<'_> {
    type Item = (usize, i32);

    fn next(&mut self) -> Option<Self::Item> {
        for (i, &Instruction(op, value)) in self.instructions.iter().skip(self.idx).enumerate() {
            let i = self.idx + i;
            let (terminated, acc) = match op {
                Operation::NOP => {
                    if -value > (i as i32) {
                        (false, 0)
                    } else {
                        let mut altered = self.instructions.to_vec();
                        altered[i] = Instruction(Operation::JMP, value);
                        (self.run_program)(&altered)
                    }
                }
                Operation::JMP => {
                    let mut altered = self.instructions.to_vec();
                    altered[i] = Instruction(Operation::NOP, value);
                    (self.run_program)(&altered)
                }
                _ => (false, 0),
            };

            if terminated {
                self.idx = i + 1;
                return Some((i, acc));
            }
        }

        None
    }
}
