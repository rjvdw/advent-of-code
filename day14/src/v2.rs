use std::collections::HashMap;

use crate::constants::{ADDRESS_SIZE, INVERT_USIZE};
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
                        for addr in apply_mask(*address, mask) {
                            memory.insert(addr, *value);
                        }
                    }
                    None => return Err(()),
                };
            }
        }
    }

    Ok(())
}

fn apply_mask(address: usize, mask: &str) -> Vec<usize> {
    let mut addresses = vec![address];

    for (idx, ch) in mask.chars().enumerate() {
        let bit_mask = 1 << (ADDRESS_SIZE - 1 - idx);
        let mut next = vec![];
        for address in addresses {
            match ch {
                '0' => {
                    next.push(address);
                }
                '1' => {
                    next.push(address | bit_mask);
                }
                'X' => {
                    next.push(address & (bit_mask ^ INVERT_USIZE));
                    next.push(address | bit_mask);
                }
                _ => panic!("Invalid character"),
            }
        }
        addresses = next;
    }

    addresses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() {
        let instructions = vec![
            Instruction::SetMask("000000000000000000000000000000X1001X".to_string()),
            Instruction::WriteValue(42, 100),
            Instruction::WriteValue(128, 200),
            Instruction::SetMask("00000000000000000000000000000000X0XX".to_string()),
            Instruction::WriteValue(26, 1),
        ];
        let mut memory: HashMap<usize, u64> = HashMap::new();

        run_program(&instructions, &mut memory).unwrap();

        assert_eq!(memory.len(), 14);
        assert_eq!(memory[&16], 1);
        assert_eq!(memory[&17], 1);
        assert_eq!(memory[&18], 1);
        assert_eq!(memory[&19], 1);
        assert_eq!(memory[&24], 1);
        assert_eq!(memory[&25], 1);
        assert_eq!(memory[&26], 1);
        assert_eq!(memory[&27], 1);
        assert_eq!(memory[&58], 100);
        assert_eq!(memory[&59], 100);
        assert_eq!(memory[&146], 200);
        assert_eq!(memory[&147], 200);
        assert_eq!(memory[&178], 200);
        assert_eq!(memory[&179], 200);
    }

    #[test]
    fn test_apply_mask_1() {
        let address = 42;
        let mask = "000000000000000000000000000000X1001X".to_string();

        assert_eq!(apply_mask(address, &mask), vec![26, 27, 58, 59]);
    }

    #[test]
    fn test_apply_mask_2() {
        let address = 26;
        let mask = "00000000000000000000000000000000X0XX".to_string();

        assert_eq!(
            apply_mask(address, &mask),
            vec![16, 17, 18, 19, 24, 25, 26, 27],
        );
    }

    #[test]
    fn test_apply_mask_3() {
        let address = 0b100000000000000000000000000000000000;
        let mask = "000000000000000000000000000000000000".to_string();

        assert_eq!(
            apply_mask(address, &mask),
            vec![0b100000000000000000000000000000000000],
        );
    }

    #[test]
    fn test_apply_mask_4() {
        let address = 0b000000000000000000000000000000000000;
        let mask = "100000000000000000000000000000000000".to_string();

        assert_eq!(
            apply_mask(address, &mask),
            vec![0b100000000000000000000000000000000000],
        );
    }

    #[test]
    fn test_apply_mask_5() {
        let address = 0b000000000000000000000000000000000000;
        let mask = "X00000000000000000000000000000000000".to_string();

        assert_eq!(
            apply_mask(address, &mask),
            vec![
                0b000000000000000000000000000000000000,
                0b100000000000000000000000000000000000,
            ],
        );
    }
}
