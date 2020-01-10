//! `error` contains the HTTP API error types.

pub struct ErrorData {
    pub code: Vec<String>,
    pub message: String,
}

pub struct ErrorResponse {
    pub non_field_errors: Vec<ErrorData>,
    pub field_errors: Vec<ErrorData>,
}
