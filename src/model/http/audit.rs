//! `audit` defines the models for interacting with
//! the HTTP Audit API.

use serde::{Deserialize, Serialize};

use crate::model::common::*;

/// `Blocks` represents blocks information.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Blocks {
    pub confirmed: Vec<Block>,
    pub latest: Block,
}

/// `SwapAudit` represents swaps information.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SwapAudit {
    pub amount: u64,
    pub amount_swapped: u64,
    pub remaining_in: u64,
    pub remaining_out: u64,
}

/// `TokenOrderBook` represents token order book information.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TokenOrderBook {
    pub buy_orders: Vec<SwapAudit>,
    pub sell_orders: Vec<SwapAudit>,
}

/// `TokenMatching` represents token matching information.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TokenMatching {
    pub price: u64,
    pub time: u64,
    pub volume: u64,
}

/// `ConciseTransfer` is a concise representation of a transfer.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConciseTransfer {
    pub amount: u64,
    pub amount_swapped: u64,
    pub cancelled: bool,
    pub complete: bool,
    pub eon_number: i64,
    pub id: i64,
    pub nonce: u64,
    pub passive: bool,
    pub recipient: Wallet,
    pub swap: bool,
    pub time: DateTime,
    pub voided: bool,
    pub wallet: Wallet,
}

/// `Transfers` is an aggregated view of transfers.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Transfers {
    pub count: i64,
    pub next: Option<Uri>,
    pub previous: Option<Uri>,
    pub results: Vec<ConciseTransfer>,
}

/// `Transfer` is a detailed view of a transfer.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Transfer {
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

/// `Deposit` is a view of a deposit.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Deposit {
    pub amount: u64,
    pub block: i64,
    pub eon_number: i64,
    pub time: DateTime,
    pub txid: String,
}

/// `Registration` represents a wallet registration data.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Registration {
    pub eon_number: u64,
    pub operator_signature: String,
    pub trail_identifier: u64,
    pub wallet_signature: String,
}

/// `WalletState` represents the state of a wallet.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WalletState {
    pub deposits: Vec<Deposit>,
    pub merkle_proofs: Vec<MerkleProof>,
    pub registration: Registration,
    pub transfers: Vec<TransferAudit>,
    pub withdrawal_request: Vec<WithdrawalRequest>,
    pub withdrawals: Vec<Withdrawal>,
}

///  `WhoIs` represents wallet admission data.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WhoIs {
    pub eon_number: u64,
    pub operator_signature: String,
    pub trail_identifier: u64,
    pub wallet_signature: String,
}
