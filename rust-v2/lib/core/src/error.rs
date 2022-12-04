//! Error handling.

use std::fmt;

/// Generic parsing error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParseError(pub String);

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

impl From<std::char::ParseCharError> for ParseError {
    fn from(err: std::char::ParseCharError) -> Self {
        ParseError(format!("{:?}", err))
    }
}

impl std::error::Error for ParseError {}

/// Macro to produce a ParseError, with optional interpolation (using format!).
#[macro_export]
macro_rules! parse_error {
    ($err:expr) => {{
        $crate::error::ParseError($err.to_string())
    }};
    ($err:expr, $($args:tt)*) => {{
        $crate::error::ParseError(format!($err, $($args)*))
    }};
}

/// Macro to produce a ParseError wrapped in an Err, with optional interpolation (using format!).
#[macro_export]
macro_rules! err_parse_error {
    ($err:expr) => {{
        Err($crate::parse_error!($err))
    }};
    ($err:expr, $($args:tt)*) => {{
        Err($crate::parse_error!($err, $($args)*))
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_parse_error_simple() {
        let err = parse_error!["This is a simple error."];
        assert_eq!(err, ParseError("This is a simple error.".to_string()));
    }

    #[test]
    fn test_macro_parse_error_with_formatting() {
        let err = parse_error!["This is an error with {}.", "formatting"];
        assert_eq!(
            err,
            ParseError("This is an error with formatting.".to_string())
        );
    }

    #[test]
    fn test_macro_err_parse_error_simple() {
        let err: Result<(), ParseError> = err_parse_error!["This is a simple error."];
        assert_eq!(err, Err(ParseError("This is a simple error.".to_string())));
    }

    #[test]
    fn test_macro_err_parse_error_with_formatting() {
        let err: Result<(), ParseError> =
            err_parse_error!["This is an error with {}.", "formatting"];

        assert_eq!(
            err,
            Err(ParseError("This is an error with formatting.".to_string()))
        );
    }
}
