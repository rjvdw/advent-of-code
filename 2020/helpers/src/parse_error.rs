use std::fmt;

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
