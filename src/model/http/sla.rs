//! `sla` defines the models for interacting with
//! the HTTP SLA API.

use crate::model::common::DateTime;

pub struct SLA {
    pub cost: u64,
    pub limit: u64,
    pub recipient: String,
    pub token: String,
}

pub struct WalletSLA {
    pub expiry: DateTime,
}
