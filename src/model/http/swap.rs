//! `swap` defines the models for interacting with
//! the HTTP Swap API.

use crate::model::common::{SenderActiveState, Signature, Wallet};

pub struct Swap {
    pub amount: u64,
    pub amount_swapped: Option<u64>,
    pub credit_balance_signature: Vec<Signature>,
    pub credit_signature: Signature,
    pub debit_balance_signature: Vec<Signature>,
    pub debit_signature: Vec<Signature>,
    pub eon_number: u64,
    pub final_receipt_hashes: Option<String>,
    pub final_receipt_index: i64,
    pub fulfillment_signature: Vec<Signature>,
    pub nonce: u64,
    pub recipient: Wallet,
    pub recipient_active_state: SenderActiveState,
    pub sender_active_state: SenderActiveState,
    pub wallet: Wallet,
}

pub struct SwapCancellation {
    pub recipient_cancellation_signature: Vec<Signature>,
    pub sender_cancellation_signature: Vec<Signature>,
}

pub struct SwapFinalization {
    pub finalization_signature: Vec<Signature>,
}

pub struct SwapFreeze {
    pub freezing_signature: Vec<Signature>,
}
