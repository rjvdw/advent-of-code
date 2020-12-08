use crate::input_record::{InputRecord, Operation};

pub struct ProgramFixer<'a> {
    instructions: &'a Vec<InputRecord>,
    solve: fn(&Vec<InputRecord>) -> (bool, i32),
    idx: usize,
}

impl ProgramFixer<'_> {
    pub fn new(instructions: &Vec<InputRecord>, solve: fn(&Vec<InputRecord>) -> (bool, i32)) -> ProgramFixer {
        ProgramFixer { instructions, solve, idx: 0 }
    }
}

impl Iterator for ProgramFixer<'_> {
    type Item = (usize, i32);

    fn next(&mut self) -> Option<Self::Item> {
        for (i, instruction) in self.instructions.iter().skip(self.idx).enumerate() {
            let i = self.idx + i;
            let (terminated, acc) = match instruction.op {
                Operation::NOP => {
                    if -instruction.value > (i as i32) {
                        (false, 0)
                    } else {
                        let mut altered = self.instructions.clone();
                        altered[i] = InputRecord { op: Operation::JMP, value: instruction.value };
                        (self.solve)(&altered)
                    }
                }
                Operation::JMP => {
                    let mut altered = self.instructions.clone();
                    altered[i] = InputRecord { op: Operation::NOP, value: instruction.value };
                    (self.solve)(&altered)
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
