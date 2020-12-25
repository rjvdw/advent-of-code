use std::collections::HashMap;

use crate::instruction::Instruction;

pub fn run_program(
    instructions: &[Instruction],
    memory: &mut HashMap<usize, u64>,
) -> Result<(), ()> {
    let mut mask: Option<(u64, u64)> = None;

    for instruction in instructions {
        match instruction {
            Instruction::SetMask(or_mask, and_mask) => {
                mask = Some((*or_mask, *and_mask));
            }
            Instruction::WriteValue(address, value) => {
                match mask {
                    Some(mask) => {
                        memory.insert(*address, apply_mask(*value, mask));
                    }
                    None => return Err(()),
                };
            }
        }
    }

    Ok(())
}

fn apply_mask(value: u64, (or_mask, and_mask): (u64, u64)) -> u64 {
    (value & and_mask) | or_mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_mask_1() {
        let value = 11;
        let mask = (
            //XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            0b000000000000000000000000000001000000,
            0b111111111111111111111111111111111101,
        );

        assert_eq!(apply_mask(value, mask), 73);
    }

    #[test]
    fn test_apply_mask_2() {
        let value = 101;
        let mask = (
            //XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            0b000000000000000000000000000001000000,
            0b111111111111111111111111111111111101,
        );

        assert_eq!(apply_mask(value, mask), 101);
    }

    #[test]
    fn test_apply_mask_3() {
        let value = 0;
        let mask = (
            //XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            0b000000000000000000000000000001000000,
            0b111111111111111111111111111111111101,
        );

        assert_eq!(apply_mask(value, mask), 64);
    }
}
