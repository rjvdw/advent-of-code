use std::fmt;
use std::process::exit;

/// Trait that adds the `or_exit_with`  method to a struct. To be used to create a way to retrieve a
/// value, or exit with a specified error code.
pub trait WithOrExit<T> {
    fn or_exit_with(self, exit_code: i32) -> T;
}

impl<T, E: fmt::Debug> WithOrExit<T> for Result<T, E> {
    /// Unwraps the value from a Result, or terminates the process with the specified exit code.
    fn or_exit_with(self, exit_code: i32) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{:?}", e);
                exit(exit_code);
            }
        }
    }
}

impl<T> WithOrExit<T> for Option<T> {
    fn or_exit_with(self, exit_code: i32) -> T {
        match self {
            Some(v) => v,
            None => {
                eprintln!("Empty option");
                exit(exit_code);
            }
        }
    }
}

/// Generic parsing error.
#[derive(Debug, PartialEq)]
pub struct ParseError(pub String);

impl ParseError {
    pub fn of(s: &str) -> ParseError {
        ParseError(s.to_string())
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError(format!("{:?}", err))
    }
}

impl From<std::num::ParseIntError> for ParseError {
    fn from(err: std::num::ParseIntError) -> Self {
        ParseError(format!("{:?}", err))
    }
}

// TODO:
// impl From<std::option::NoneError> for ParseError {
//     fn from(err: std::option::NoneError) -> Self {
//         ParseError(format!("{:?}", err))
//     }
// }
