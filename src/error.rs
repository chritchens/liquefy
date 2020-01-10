//! `error` defines the error type used in the library.

use http::uri;
use rug::integer;
use serde_json as json;
use thiserror::Error;

use std::io;

/// `Error` is the error type used in the library.
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {source:?}")]
    IO {
        #[from]
        source: io::Error,
    },
    #[error("(JSON error: {source:?}")]
    JSON {
        #[from]
        source: json::Error,
    },
    #[error("Invalid URI: {source:?}")]
    InvalidURI {
        #[from]
        source: uri::InvalidUri,
    },
    #[error("Invalid decimal: {source:?}")]
    InvalidDecimal {
        #[from]
        source: integer::ParseIntegerError,
    },
    #[error("Invalid length {length:?} when {expected:?} was expected")]
    InvalidLength { length: usize, expected: usize },
    #[error("Value {value:?} out of range {min:?}-{max:?}")]
    OutOfRange {
        value: i64,
        min: Option<i64>,
        max: Option<i64>,
    },
    #[error("Missing field {field:?}")]
    MissingField { field: String },
    #[error("Unknown field {field:?}")]
    UnknownField { field: String },
}
