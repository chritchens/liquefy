//! `sla` defines the models for interacting with
//! the HTTP SLA (Service Level Agreement) API.

use serde::{Deserialize, Serialize};

use crate::model::common::DateTime;

/// `SLA` represents a Service Level Agreement.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SLA {
    pub cost: u64,
    pub limit: u64,
    pub recipient: String,
    pub token: String,
}

/// `WalletSLA` is a Wallet SLA
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WalletSLA {
    pub expiry: DateTime,
}
