//! `common` contains the common models.

use chrono::Utc;
use http::uri::Uri;
use uuid::Uuid;

/// `DateTime` is a UTC datetime.
pub struct DateTime(chrono::DateTime<Utc>);

/// `UUID` is a v4 UUID.
pub struct UUID(Uuid);

/// `URI` is an RFC3986 URI.
pub struct URI(Uri);

pub struct Wallet {
    pub address: String,
    pub token: String,
}

pub struct Block {
    pub block: u64,
    pub eon_number: u64,
}

pub struct SenderActiveState {
    pub operator_signature: String,
    pub tx_set_hash: String,
    pub tx_set_index: String,
    pub tx_set_proof: Vec<String>,
    pub updated_gains: String,
    pub updated_spendings: String,
    pub wallet_signature: String,
}

pub struct Signature {
    pub value: String,
}
