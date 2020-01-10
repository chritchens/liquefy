//! `admission` defines the models for interacting with
//! the HTTP Admission API.

pub struct Admission {
    pub address: String,
    pub authorization: String,
    pub token: String,
}
