use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use shared::intcode::parse::parse_input;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let program = parse_input(file).or_exit_with(1);

    let mut test_program = program.clone();
    test_program.send_message(1);
    test_program.run();
    match test_program.receive_message() {
        Some(boost) => println!("The BOOST keycode is {}.", boost),
        None => eprintln!("No BOOST keycode received."),
    }

    let mut boost_program = program;
    boost_program.send_message(2);
    boost_program.run();
    match boost_program.receive_message() {
        Some(coordinates) => println!(
            "The coordinates of the distress signal are {}.",
            coordinates
        ),
        None => eprintln!("No coordinates received."),
    }
}
