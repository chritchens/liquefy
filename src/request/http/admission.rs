//! `admission` defines the functions for interacting with
//! the HTTP Admission API.

pub const ADMISSION_CREATE_ERROR_CODES: &'static [&'static str] = &[
    "TOKEN_NOT_REGISTERED",
    "WALLET_BLACKLISTED",
    "WALLET_ALREADY_ADMITTED",
    "INVALID_ADMISSION_SIGNATURE",
];

pub const ADMISSION_CREATE_BULK_ERROR_CODES: &'static [&'static str] = &[
    "TOO_MANY_ADMISSION_REQUESTS",
    "TOKEN_NOT_REGISTERED",
    "INVALID_ADMISSION_SIGNATURE",
];
