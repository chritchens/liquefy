//! `common` contains the common models.

use crate::error::*;
use chrono::Utc;
use http::uri;
use rug::Integer;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;
use uuid;

/// `DateTime` is a UTC datetime.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DateTime(chrono::DateTime<Utc>);

/// `Uid` is a v4 UUID.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Uuid(uuid::Uuid);

/// `Uri` is an RFC3986 URI.
#[derive(Clone, Eq, PartialEq)]
pub struct Uri(uri::Uri);

impl FromStr for Uri {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        uri::Uri::from_str(s)
            .map(|uri| Uri(uri))
            .map_err(|e| e.into())
    }
}

impl ToString for Uri {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Serialize for Uri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct UriVisitor;

impl<'de> Visitor<'de> for UriVisitor {
    type Value = Uri;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid Uri string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Uri::from_str(value).map_err(|e| E::custom(e.to_string()))
    }
}

impl<'de> Deserialize<'de> for Uri {
    fn deserialize<D>(deserializer: D) -> Result<Uri, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(UriVisitor)
    }
}

/// `Decimal` is a big integer value.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Decimal(Integer);

impl FromStr for Decimal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        rug::Integer::from_str(s)
            .map(|bigint| Decimal(bigint))
            .map_err(|e| e.into())
    }
}

impl ToString for Decimal {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

/// `Wallet` represent a wallet coordinate.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub token: String,
}

/// `Block` represents a block coordinate.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub block: u64,
    pub eon_number: u64,
}

/// `SenderActiveState` represents a sender state.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SenderActiveState {
    pub operator_signature: String,
    pub tx_set_hash: String,
    pub tx_set_index: String,
    pub tx_set_proof: Vec<String>,
    pub updated_gains: String,
    pub updated_spendings: String,
    pub wallet_signature: String,
}

/// `ActiveState` represents an active state. // TODO: improve
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ActiveState {
    pub operator_signature: String,
    pub tx_set_hash: String,
    pub updated_gains: String,
    pub updated_spendings: String,
    pub wallet_signature: String,
}

/// `MerkleProof` represents a Merkle proof.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MerkleProof {
    pub active_state: ActiveState,
    pub active_state_checksum: String,
    pub allotment_chain: Vec<String>,
    pub eon_number: u64,
    pub left: u64,
    pub membership_chain: Vec<String>,
    pub passive_amount: u64,
    pub passive_checksum: String,
    pub passive_marker: String,
    pub right: u64,
    pub trail: u64,
    pub values: Vec<u64>,
}

/// `DeliveryProof` represents a proof of delivery.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DeliveryProof {
    pub merkle_proof: MerkleProof,
    pub transfer_membership_chain: Vec<String>,
    pub transfer_membership_trail: u64,
    pub transfer_membership_values: Vec<u64>,
}

/// `MatchedAmounts` represents a match of swapped amounts.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MatchedAmounts {
    pub r#in: u64,
    pub matched_in: u64,
    pub matched_out: u64,
    pub out: u64,
}

/// `TokenMatching` represents token information.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub address: String,
    pub name: Option<String>,
    pub short_name: Option<String>,
}

/// `TransferAudit` represents a transfer audit receipt.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TransferAudit {
    pub amount: u64,
    pub amount_swapped: Option<u64>,
    pub appended: bool,
    pub cancelled: bool,
    pub complete: bool,
    pub delivery_proof: DeliveryProof,
    pub eon_number: i64,
    pub id: i64,
    pub matched_amounts: MatchedAmounts,
    pub nonce: u64,
    pub passive: bool,
    pub position: Option<u64>,
    pub processed: bool,
    pub recipient: Wallet,
    pub recipient_active_state: SenderActiveState,
    pub recipient_cancellation_active_state: SenderActiveState,
    pub recipient_finalization_active_state: SenderActiveState,
    pub recipient_fulfillment_active_state: SenderActiveState,
    pub recipient_starting_balance: Option<u64>,
    pub recipient_trail_identifier: i64,
    pub sender_active_state: SenderActiveState,
    pub sender_cancellation_active_state: SenderActiveState,
    pub sender_finalization_active_state: SenderActiveState,
    pub sender_starting_balance: Option<u64>,
    pub swap_freezing_signature: String,
    pub time: DateTime,
    pub timestamp: u64,
    pub tx_id: Option<Uuid>,
    pub voided: bool,
    pub wallet: Wallet,
    pub wallet_trail_identifier: i64,
}

/// `WithdrawalRequest` represents a withdrawal request.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WithdrawalRequest {
    pub amount: u64,
    pub block: i64,
    pub eon_number: i64,
    pub slashed: bool,
    pub time: DateTime,
    pub txid: String,
}

/// `Withdrawal` represents a withdrawal.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Withdrawal {
    pub amount: u64,
    pub block: i64,
    pub eon_number: i64,
    pub request: WithdrawalRequest,
    pub time: DateTime,
    pub txid: String,
}

/// Signature represents a signature.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Signature {
    pub value: String,
}
