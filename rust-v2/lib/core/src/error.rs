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

impl From<()> for ParseError {
    fn from(_value: ()) -> Self {
        ParseError("could not parse".to_string())
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

impl From<std::num::ParseFloatError> for ParseError {
    fn from(err: std::num::ParseFloatError) -> Self {
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
    () => {{
        $crate::error::ParseError::from(())
    }};
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
    () => {{
        Err($crate::parse_error!())
    }};
    ($err:expr) => {{
        Err($crate::parse_error!($err))
    }};
    ($err:expr, $($args:tt)*) => {{
        Err($crate::parse_error!($err, $($args)*))
    }};
}

/// Macro to make an assertion, or return a ParseError wrapped in an Err, with optional interpolation (using format!).
#[macro_export]
macro_rules! assert_or_parse_error {
    ($pred: expr) => {{
        if !($pred) {
            $crate::err_parse_error!()?
        }
    }};
    ($pred: expr, $err:expr) => {{
        if !($pred) {
            $crate::err_parse_error!($err)?
        }
    }};
    ($pred: expr, $err:expr, $($args:tt)*) => {{
        if !($pred) {
            $crate::err_parse_error!($err, $($args)*)?
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParseResult;

    #[test]
    fn test_macro_parse_error_no_args() {
        let err = parse_error![];
        assert_eq!(err, ParseError::from(()));
    }

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
    fn test_macro_err_parse_error_no_args() {
        let err: ParseResult<()> = err_parse_error![];
        assert_eq!(err, Err(ParseError::from(())));
    }

    #[test]
    fn test_macro_err_parse_error_simple() {
        let err: ParseResult<()> = err_parse_error!["This is a simple error."];
        assert_eq!(err, Err(ParseError("This is a simple error.".to_string())));
    }

    #[test]
    fn test_macro_err_parse_error_with_formatting() {
        let err: ParseResult<()> = err_parse_error!["This is an error with {}.", "formatting"];

        assert_eq!(
            err,
            Err(ParseError("This is an error with formatting.".to_string()))
        );
    }

    #[test]
    fn test_macro_assert_or_parse_error_assertion_true_and_no_args() {
        fn f() -> ParseResult<()> {
            assert_or_parse_error!(true);
            Ok(())
        }

        assert_eq!(f(), Ok(()));
    }

    #[test]
    fn test_macro_assert_or_parse_error_assertion_false_and_no_args() {
        fn f() -> ParseResult<()> {
            assert_or_parse_error!(false);
            Ok(())
        }

        assert_eq!(f(), Err(ParseError::from(())));
    }

    #[test]
    fn test_macro_assert_or_parse_error_assertion_true_and_simple() {
        fn f() -> ParseResult<()> {
            assert_or_parse_error!(true, "This is a simple error");
            Ok(())
        }

        assert_eq!(f(), Ok(()));
    }

    #[test]
    fn test_macro_assert_or_parse_error_assertion_false_and_simple() {
        fn f() -> ParseResult<()> {
            assert_or_parse_error!(false, "This is a simple error.");
            Ok(())
        }

        assert_eq!(f(), Err(ParseError("This is a simple error.".to_string())));
    }

    #[test]
    fn test_macro_assert_or_parse_error_assertion_true_and_with_formatting() {
        fn f() -> ParseResult<()> {
            assert_or_parse_error!(true, "This is an error with {}.", "formatting");
            Ok(())
        }

        assert_eq!(f(), Ok(()));
    }

    #[test]
    fn test_macro_assert_or_parse_error_assertion_false_and_with_formatting() {
        fn f() -> ParseResult<()> {
            assert_or_parse_error!(false, "This is an error with {}.", "formatting");
            Ok(())
        }

        assert_eq!(
            f(),
            Err(ParseError("This is an error with formatting.".to_string()))
        );
    }
}
