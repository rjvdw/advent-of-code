use std::fmt;
use std::process::exit;

pub mod from_multiline_str;
pub mod parse;
pub mod parse_error;
pub mod part;
pub mod read;

/// Helper method for dealing with results. If a result is an Err, it will print an error message
/// and terminate the process with an exit code of 1.
pub fn handle_result<T, E: fmt::Display>(res: Result<T, E>) -> T {
    match res {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
