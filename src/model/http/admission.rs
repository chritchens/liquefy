//! `admission` defines the models for interacting with
//! the HTTP Admission API.

use serde::{Deserialize, Serialize};

/// `Admission` represents a wallet admission information.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Admission {
    pub address: String,
    pub authorization: String,
    pub token: String,
}
