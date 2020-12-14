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
                    Some((or_mask, and_mask)) => {
                        let or_mask = or_mask as usize;
                        let and_mask = and_mask as usize;
                        let address = address | or_mask;
                        for addr in apply_mask(address, or_mask, and_mask) {
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

fn apply_mask(address: usize, or_mask: usize, and_mask: usize) -> Vec<usize> {
    let mut xor = or_mask ^ and_mask;
    let mut masks = vec![address];
    let mut i: usize = 1;

    while xor != 0 {
        let bit = xor % (1 << i);
        if bit != 0 {
            xor ^= bit;
            masks = masks
                .iter()
                .flat_map(|x| vec![x | bit, x & (bit ^ usize::MAX)])
                .collect::<Vec<usize>>();
        }
        i += 1;
    }

    masks.sort_unstable();
    masks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_masks_1() {
        let address = 42;
        let or_mask = 0b000000000000000000000000000000010010;
        let and_mask = 0b000000000000000000000000000000110011;

        assert_eq!(
            apply_mask(address | or_mask, or_mask, and_mask),
            vec![26, 27, 58, 59]
        );
    }

    #[test]
    fn get_masks_2() {
        let address = 26;
        let or_mask = 0b000000000000000000000000000000000000;
        let and_mask = 0b000000000000000000000000000000001011;

        assert_eq!(
            apply_mask(address, or_mask, and_mask),
            vec![16, 17, 18, 19, 24, 25, 26, 27]
        );
    }
}
