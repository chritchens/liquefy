//! `result` defines the Result type used in the library.

use crate::error::Error;

/// `Result` is the result type of the library.
pub type Result<T, E = Error> = std::result::Result<T, E>;
