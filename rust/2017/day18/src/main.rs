use std::collections::VecDeque;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;
use rdcl_aoc_helpers::machine::instruction::MachineInstruction;
use rdcl_aoc_helpers::machine::output_receiver::OutputReceiver;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::machine::Machine;

use crate::aborting_hook::AbortingHook;
use crate::instruction::Instruction;

mod aborting_hook;
mod instruction;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let instructions: Vec<Instruction> = File::open(&args[1]).read_lines(1).collect();

    let mut machine = Machine::new_simple_machine(&instructions);
    let mut pre_execute_hook = AbortingHook::new();
    machine.run(&mut pre_execute_hook);

    match pre_execute_hook.get_first_recovered_sound() {
        Some(sound) => println!("The first recovered sound was {}.", sound),
        None => println!("No sound was ever recovered."),
    }

    let (_, send_count_1) = run_2_programs(&instructions);
    println!(
        "Program 1 sent {} values before reaching deadlock.",
        send_count_1
    );
}

fn run_2_programs(instructions: &[Instruction]) -> (usize, usize) {
    let mut machine_0 = Machine::new_simple_machine(instructions);
    let mut program_counter_0: i64 = 0;
    let mut message_bus_0: VecDeque<i64> = VecDeque::new();
    let mut send_count_0 = 0;
    machine_0.register.write('_', 0);
    machine_0.register.write('p', 0);

    let mut machine_1 = Machine::new_simple_machine(instructions);
    let mut program_counter_1: i64 = 0;
    let mut message_bus_1: VecDeque<i64> = VecDeque::new();
    let mut send_count_1 = 0;
    machine_1.register.write('_', 1);
    machine_1.register.write('p', 1);

    loop {
        let r1 = run(
            &mut machine_0,
            &mut program_counter_0,
            &mut message_bus_1,
            &mut message_bus_0,
            &mut send_count_0,
        );
        let r2 = run(
            &mut machine_1,
            &mut program_counter_1,
            &mut message_bus_0,
            &mut message_bus_1,
            &mut send_count_1,
        );

        if !r1 && !r2 {
            return (send_count_0, send_count_1);
        }
    }
}

fn run<R, O>(
    machine: &mut Machine<Instruction, R, O>,
    idx: &mut i64,
    bus_send: &mut VecDeque<i64>,
    bus_receive: &mut VecDeque<i64>,
    send_count: &mut usize,
) -> bool
where
    R: MachineRegister,
    O: OutputReceiver<R>,
{
    if let Some((_, instruction)) = machine.get_instruction(*idx) {
        let mut did_something = true;
        *idx += match instruction {
            Instruction::Sound(a) => {
                *send_count += 1;
                bus_send.push_back(a.get(&machine.register));
                1
            }
            Instruction::Recover(a) => {
                if let Some(value) = bus_receive.pop_front() {
                    machine.register.write(a, value);
                    1
                } else {
                    did_something = false;
                    0
                }
            }
            _ => instruction.execute(&mut machine.register, &mut machine.output_receiver),
        };
        did_something
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_program() {
        let instructions = vec![
            "set a 1", "add a 2", "mul a a", "mod a 5", "snd a", "set a 0", "rcv a", "jgz a -1",
            "set a 1", "jgz a -2",
        ]
        .as_records::<Instruction>()
        .unwrap();

        let mut machine = Machine::new_simple_machine(&instructions);
        let mut pre_execute_hook = AbortingHook::new();
        machine.run(&mut pre_execute_hook);

        assert_eq!(pre_execute_hook.get_first_recovered_sound(), Some(4));
    }

    #[test]
    fn test_2_programs() {
        let instructions = vec![
            "snd 1", "snd 2", "snd p", "rcv a", "rcv b", "rcv c", "rcv d",
        ]
        .as_records::<Instruction>()
        .unwrap();

        assert_eq!(run_2_programs(&instructions), (3, 3));
    }
}
