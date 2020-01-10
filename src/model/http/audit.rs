//! `audit` defines the models for interacting with
//! the HTTP Audit API.

use crate::model::common::{Block, DateTime, SenderActiveState, Wallet, URI, UUID};

pub struct Blocks {
    pub confirmed: Vec<Block>,
    pub latest: Block,
}

pub struct SwapAudit {
    pub amount: u64,
    pub amount_swapped: u64,
    pub remaining_in: u64,
    pub remaining_out: u64,
}

pub struct TokenOrderBook {
    pub buy_orders: Vec<SwapAudit>,
    pub sell_orders: Vec<SwapAudit>,
}

pub struct TokenMatching {
    pub price: u64,
    pub time: u64,
    pub volume: u64,
}

pub struct Token {
    pub address: String,
    pub name: Option<String>,
    pub short_name: Option<String>,
}

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

pub struct Transfers {
    pub count: i64,
    pub next: Option<URI>,
    pub previous: Option<URI>,
    pub results: Vec<ConciseTransfer>,
}

pub struct ActiveState {
    pub operator_signature: String,
    pub tx_set_hash: String,
    pub updated_gains: String,
    pub updated_spendings: String,
    pub wallet_signature: String,
}

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

pub struct DeliveryProof {
    pub merkle_proof: MerkleProof,
    pub transfer_membership_chain: Vec<String>,
    pub transfer_membership_trail: u64,
    pub transfer_membership_values: Vec<u64>,
}

pub struct MatchedAmounts {
    pub r#in: u64,
    pub matched_in: u64,
    pub matched_out: u64,
    pub out: u64,
}

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
    pub tx_id: Option<UUID>,
    pub voided: bool,
    pub wallet: Wallet,
    pub wallet_trail_identifier: i64,
}

pub struct Deposit {
    pub amount: u64,
    pub block: i64,
    pub eon_number: i64,
    pub time: DateTime,
    pub txid: String,
}

pub struct Registration {
    pub eon_number: u64,
    pub operator_signature: String,
    pub trail_identifier: u64,
    pub wallet_signature: String,
}

// NB: should be accessible to the ws module
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
    pub tx_id: Option<UUID>,
    pub voided: bool,
    pub wallet: Wallet,
    pub wallet_trail_identifier: i64,
}

// NB: should be accessible to the ws module
pub struct WithdrawalRequest {
    pub amount: u64,
    pub block: i64,
    pub eon_number: i64,
    pub slashed: bool,
    pub time: DateTime,
    pub txid: String,
}

// NB: should be accessible to the ws module
pub struct Withdrawal {
    pub amount: u64,
    pub block: i64,
    pub eon_number: i64,
    pub request: WithdrawalRequest,
    pub time: DateTime,
    pub txid: String,
}

pub struct WalletState {
    pub deposits: Vec<Deposit>,
    pub merkle_proofs: Vec<MerkleProof>,
    pub registration: Registration,
    pub transfers: Vec<TransferAudit>,
    pub withdrawal_request: Vec<WithdrawalRequest>,
    pub withdrawals: Vec<Withdrawal>,
}

pub struct WhoIs {
    pub eon_number: u64,
    pub operator_signature: String,
    pub trail_identifier: u64,
    pub wallet_signature: String,
}
