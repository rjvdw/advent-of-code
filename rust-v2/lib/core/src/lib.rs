extern crate core;

use std::error;

pub mod input;

/// Type alias for results that may contain any error
pub type DynResult<T> = Result<T, Box<dyn error::Error>>;

/// Type alias for the return type of the main method
pub type MainResult = DynResult<()>;
