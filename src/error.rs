//! `error` defines the error type used in the library.

use http::uri;
use std::io;
use thiserror::Error;

/// `Error` is the error type used in the library.
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {source:?}")]
    IO {
        #[from]
        source: io::Error,
    },
    #[error("Invalid URI: {source:?}")]
    InvalidURI {
        #[from]
        source: uri::InvalidUri,
    },
}
