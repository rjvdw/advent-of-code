mod input_record;

extern crate helpers;

use std::env;
use std::process::exit;
use helpers::{read_input, handle_result};
use input_record::InputRecord;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input file>", &args[0]);
        exit(1);
    }

    let path = &args[1];
    let values: Vec<InputRecord> = handle_result(read_input(path));

    println!("Number of valid passwords according to old job: {}", values.iter()
        .filter(|v| v.valid_according_to_old_job()).count());
    println!("Number of valid passwords according to corporate policy: {}", values.iter()
        .filter(|v| v.valid_according_to_corporate_policy()).count());
}
