use std::collections::HashMap;

use crate::constants::INVERT_U64;
use crate::instruction::Instruction;

pub fn run_program(
    instructions: &[Instruction],
    memory: &mut HashMap<usize, u64>,
) -> Result<(), ()> {
    let mut mask: Option<&String> = None;

    for instruction in instructions {
        match instruction {
            Instruction::SetMask(new_mask) => {
                mask = Some(new_mask);
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

fn apply_mask(mut value: u64, mask: &str) -> u64 {
    for (idx, ch) in mask.chars().rev().enumerate() {
        let bit_mask = 1 << idx;
        match ch {
            '0' => {
                value &= bit_mask ^ INVERT_U64;
            }
            '1' => {
                value |= bit_mask;
            }
            'X' => {}
            _ => panic!("Invalid character"),
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_mask_1() {
        let value = 11;
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string();

        assert_eq!(apply_mask(value, &mask), 73);
    }

    #[test]
    fn test_apply_mask_2() {
        let value = 101;
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string();

        assert_eq!(apply_mask(value, &mask), 101);
    }

    #[test]
    fn test_apply_mask_3() {
        let value = 0;
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string();

        assert_eq!(apply_mask(value, &mask), 64);
    }
}
