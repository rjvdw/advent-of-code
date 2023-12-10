extern crate core;

use crate::error::ParseError;

pub mod error;
pub mod input;
pub mod parser;

/// Type alias for results that may contain any error
pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Type alias for parse results
pub type ParseResult<T> = Result<T, ParseError>;

/// Type alias for the return type of the main method
pub type MainResult = DynResult<()>;
