extern crate core;

pub mod error;
pub mod input;

/// Type alias for results that may contain any error
pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Type alias for the return type of the main method
pub type MainResult = DynResult<()>;
