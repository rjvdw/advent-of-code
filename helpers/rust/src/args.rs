use std::env;
use std::process::exit;

/// Reads the command line arguments and does some very basic validation.
pub fn get_args(required_args: &[&str], error_exit_code: i32) -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != required_args.len() + 1 {
        eprintln!("Usage: {} {}", &args[0], required_args.join(" "));
        exit(error_exit_code);
    }

    args
}
