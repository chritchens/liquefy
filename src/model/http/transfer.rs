//! `transfer` defines the models for interacting with
//! the HTTP Transfer API.

use serde::{Deserialize, Serialize};

use crate::model::common::{DateTime, SenderActiveState, Signature, Uuid, Wallet};

/// `Transfer` is a detailed view of a transfer.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Transfer {
    pub amount: u64,
    pub complete: bool,
    pub eon_number: i64,
    pub final_receipt_hashes: Option<String>,
    pub final_receipt_index: Option<i64>,
    pub id: i64,
    pub nonce: u64,
    pub passive: bool,
    pub position: u64,
    pub processed: bool,
    pub recipient: String,
    pub recipient_active_state: SenderActiveState,
    pub sender_active_state: SenderActiveState,
    pub sender_finalization_active_state: SenderActiveState,
    pub time: DateTime,
    pub tx_id: Uuid,
    pub wallet: Wallet,
    pub wallet_balance: u64,
    pub wallet_balance_signature: Signature,
    pub wallet_signature: Signature,
}

/// `TransferReceipt` is the receipt of a transfer.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TransferReceipt {
    pub recipient: Wallet,
    pub recipient_active_state: SenderActiveState,
    pub sender_active_state: SenderActiveState,
    pub wallet: Wallet,
    pub wallet_signature: Signature,
}
