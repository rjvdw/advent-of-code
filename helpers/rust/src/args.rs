use std::process::exit;
use std::{env, fmt};

/// Reads the command line arguments and verifies that the correct number of arguments are present.
pub fn get_args(required_args: &[&str], error_exit_code: i32) -> Vec<String> {
    get_and_validate_args(
        |args| args.len() == required_args.len() + 1,
        required_args.join(" "),
        error_exit_code,
    )
}

/// Reads the command line arguments and verifies that there are at least the required number of
/// arguments present.
pub fn get_args_repeating(required_args: &[&str], error_exit_code: i32) -> Vec<String> {
    get_and_validate_args(
        |args| args.len() >= required_args.len() + 1,
        required_args.join(" "),
        error_exit_code,
    )
}

fn get_and_validate_args<P, E: fmt::Debug>(
    check: P,
    error_message: E,
    error_exit_code: i32,
) -> Vec<String>
where
    P: Fn(&Vec<String>) -> bool,
{
    let args: Vec<String> = env::args().collect();
    if check(&args) {
        args
    } else {
        eprintln!("Usage: {} {:?}", &args[0], error_message);
        exit(error_exit_code);
    }
}
