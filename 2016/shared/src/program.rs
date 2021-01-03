use rdcl_aoc_helpers::machine::hook::{HookResult, PreExecuteHook};
use rdcl_aoc_helpers::machine::instruction::{MachineInstruction, Value};
use rdcl_aoc_helpers::machine::output_receiver::OutputReceiver;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::machine::Machine;

use crate::instruction::Instruction;

pub struct Hook;

impl PreExecuteHook<Instruction> for Hook {
    fn run<R: MachineRegister, O: OutputReceiver<R>>(
        &mut self,
        machine: &mut Machine<Instruction, R, O>,
        instruction: &Instruction,
        _idx: usize,
    ) -> HookResult {
        if apply_optimization(machine) {
            HookResult::Goto(machine.get_counter() + 6)
        } else {
            match instruction {
                Instruction::Toggle(_) => {
                    let target_idx = machine.get_counter()
                        + instruction.execute(&mut machine.register, &mut machine.output_receiver);

                    if let Some((_, instruction)) = machine.get_instruction(target_idx) {
                        let new_instruction = match instruction {
                            Instruction::Copy(a, b) => {
                                Instruction::JumpNotZero(a, Value::Register(b))
                            }
                            Instruction::Increment(a) => Instruction::Decrement(a),
                            Instruction::Decrement(a) => Instruction::Increment(a),
                            Instruction::JumpNotZero(a, Value::Register(b)) => {
                                Instruction::Copy(a, b)
                            }
                            Instruction::Toggle(Value::Register(a)) => Instruction::Increment(a),
                            _ => panic!("cannot transform instruction '{}'", instruction),
                        };

                        machine.set_instruction(target_idx, &new_instruction);
                    }

                    HookResult::Skip
                }
                _ => HookResult::Proceed,
            }
        }
    }
}

fn apply_optimization<R, O>(machine: &mut Machine<Instruction, R, O>) -> bool
where
    R: MachineRegister,
    O: OutputReceiver<R>,
{
    // If the next 6 lines are:
    //   cpy b c
    //   inc a
    //   dec c
    //   jnz c -2
    //   dec d
    //   jnz d -5
    // we are actually doing multiplication. This can be optimized, by simply doing:
    //   * Increment register a by b*d.
    //   * Clear register c.
    //   * Clear register d.

    let idx = machine.get_counter();

    // cpy b c
    let (val_b, c) = if let Some((_, Instruction::Copy(a, b))) = machine.get_instruction(idx) {
        (a.get(&machine.register), b)
    } else {
        return false;
    };

    // inc a
    let a = if let Some((_, Instruction::Increment(a))) = machine.get_instruction(idx + 1) {
        a
    } else {
        return false;
    };

    // dec c
    if let Some((_, Instruction::Decrement(ch))) = machine.get_instruction(idx + 2) {
        if ch != c {
            return false;
        }
    } else {
        return false;
    };

    // jnz c -2
    if let Some((_, Instruction::JumpNotZero(Value::Register(ch), Value::Raw(-2)))) =
        machine.get_instruction(idx + 3)
    {
        if ch != c {
            return false;
        }
    } else {
        return false;
    };

    // dec d
    let d = if let Some((_, Instruction::Decrement(d))) = machine.get_instruction(idx + 4) {
        d
    } else {
        return false;
    };

    // jnz d -5
    if let Some((_, Instruction::JumpNotZero(Value::Register(ch), Value::Raw(-5)))) =
        machine.get_instruction(idx + 5)
    {
        if ch != d {
            return false;
        }
    } else {
        return false;
    };

    let val_d = machine.register.read(d);

    machine.register.increment(a, val_b * val_d);
    machine.register.write(c, 0);
    machine.register.write(d, 0);

    true
}
