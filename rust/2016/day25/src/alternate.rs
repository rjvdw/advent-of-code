use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;
use rdcl_aoc_helpers::machine::instruction::Value;

use shared::instruction::Instruction;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let instructions = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Instruction>>();

    match alternate_solution(&instructions) {
        Some(a) => println!("For a={}, the antenna will repeat 0,1,0,1,...", a),
        None => eprintln!("Could not find a value for a for which the antenna will repeat 0,1,0,1,... Maybe you have better luck if you do not use the alternate solution."),
    }
}

/// This is an alternate solution. It might only work for my specific input.
fn alternate_solution(instructions: &[Instruction]) -> Option<i64> {
    let i1 = if let Instruction::Copy(Value::Raw(v), _) = instructions[1] {
        v
    } else {
        return None;
    };

    let i2 = if let Instruction::Copy(Value::Raw(v), _) = instructions[2] {
        v
    } else {
        return None;
    };

    let threshold = i1 * i2;
    let mut a = 0;
    let mut t = 1;
    while a < threshold {
        a = a * 2 + t;
        t = 1 - t;
    }

    Some(a - threshold)
}
