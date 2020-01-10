//! `http` contains the types and functions for the HTTP API of the Liquid Network hub.

pub mod error {
    pub struct ErrorData {
        pub code: Vec<String>,
        pub message: String,
    }

    pub struct ErrorResponse {
        pub non_field_errors: Vec<ErrorData>,
        pub field_errors: Vec<ErrorData>,
    }
}

pub mod common {
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
}

pub mod admission {
    pub struct Admission {
        pub address: String,
        pub authorization: String,
        pub token: String,
    }

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
}

pub mod analytics {
    use crate::utils::DateTime;

    use crate::http::common::Block;

    pub struct EonSnapshot {
        pub count: u64,
        pub eon_number: u64,
    }

    pub struct DaySnapshot {
        pub count: u64,
        pub day: DateTime,
    }

    pub struct Challenges {
        pub eon_number: Vec<EonSnapshot>,
        pub rebuted: u64,
        pub time: Vec<DaySnapshot>,
    }

    pub struct Deposits {
        pub eon_number: Vec<EonSnapshot>,
        pub time: Vec<DaySnapshot>,
        pub total: u64,
    }

    pub struct OperatorStatus {
        pub blocks_per_eon: u64,
        pub confirmed: Vec<Block>,
        pub current_eon_number: u64,
        pub latest: Block,
    }

    pub struct Transfers {
        pub eon_number: Vec<EonSnapshot>,
        pub time: Vec<DaySnapshot>,
        pub total: u64,
    }

    pub struct Admissions {
        pub eon_number: Vec<EonSnapshot>,
        pub total: u64,
    }

    pub struct Withdrawals {
        pub eon_number: Vec<EonSnapshot>,
        pub time: Vec<DaySnapshot>,
        pub total: u64,
    }
}

pub mod audit {
    use crate::http::common::{Block, SenderActiveState, Wallet};
    use crate::utils::{DateTime, URI, UUID};

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
}

pub mod sla {
    use crate::utils::DateTime;

    pub struct SLA {
        pub cost: u64,
        pub limit: u64,
        pub recipient: String,
        pub token: String,
    }

    pub struct WalletSLA {
        pub expiry: DateTime,
    }
}

pub mod swap {
    use crate::http::common::{SenderActiveState, Signature, Wallet};

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

    pub const SWAP_CREATE_ERROR_CODES: &'static [&'static str] = &[
        "INVALID_DEBIT_AMOUNT",
        "INVALID_CREDIT_AMOUNT",
        "SWAPPING_DISABLED",
        "TOO_MANY_FUTURE_SIGNATURES",
        "WRONG_NUMBER_OF_SIGNATURES",
        "DEBIT_WALLET_NOT_ADMITTED",
        "CREDIT_WALLET_NOT_ADMITTED",
        "DEBIT_CREDIT_WALLET_ADDRESS_MISMATCH",
        "DEBIT_CREDIT_TOKEN_ADDRESS_MATCH",
        "TOKEN_PAIR_BLOCKED",
        "EON_NUMBER_OUT_OF_SYNC",
        "DEBIT_WALLET_CANNOT_ADD_TRANSACTION",
        "CREDIT_WALLET_CANNOT_ADD_TRANSACTION",
        "DEBIT_WALLET_OVERSPENDING",
        "DEBIT_WALLET_BALANCE_AMOUNT_MISMATCH",
        "CREDIT_WALLET_BALANCE_NOT_ZERO",
        "INVALID_DEBIT_BALANCE_SIGNATURE",
        "INVALID_CREDIT_BALANCE_SIGNATURE",
        "INVALID_FUTURE_CREDIT_SIGNATURE",
        "INVALID_CREDIT_SIGNATURE",
        "INVALID_FUTURE_DEBIT_SIGNATURE",
        "INVALID_DEBIT_SIGNATURE",
        "INVALID_FUTURE_CREDIT_FULFILLMENT_SIGNATURE",
        "INVALID_CREDIT_FULFILLMENT_SIGNATURE",
    ];

    pub struct SwapCancellation {
        pub recipient_cancellation_signature: Vec<Signature>,
        pub sender_cancellation_signature: Vec<Signature>,
    }

    pub const SWAP_CANCEL_UPDATE_ERROR_CODES: &'static [&'static str] = &[
        "WRONG_NUMBER_OF_DEBIT_SIGNATURES",
        "WRONG_NUMBER_OF_CREDIT_SIGNATURES",
        "SWAP_NOT_FROZEN",
        "SWAP_ALREADY_CLOSED",
        "MISSING_FREEZING_SIGNATURE",
        "SWAP_ALREADY_CANCELLED",
        "INVALID_CREDIT_SIGNATURE",
        "INVALID_FUTURE_CREDIT_SIGNATURE",
        "INVALID_DEBIT_SIGNATURE",
        "INVALID_FUTURE_DEBIT_SIGNATURE",
    ];

    pub struct SwapFinalization {
        pub finalization_signature: Vec<Signature>,
    }

    pub const SWAP_FINALIZE_UPDATE_ERROR_CODES: &'static [&'static str] = &[
        "SWAP_NOT_FULFILLED",
        "SWAP_ALREADY_FROZEN",
        "SWAP_ALREADY_VOIDED",
        "SWAP_ALREADY_CLOSED",
        "SWAP_ALREADY_FINALIZED",
        "WRONG_NUMBER_OF_CREDIT_SIGNATURES",
        "INVALID_CREDIT_SIGNATURE",
        "INVALID_FUTURE_CREDIT_SIGNATURE",
    ];

    pub struct SwapFreeze {
        pub freezing_signature: Vec<Signature>,
    }

    pub const SWAP_FREEZE_UPDATE_ERROR_CODES: &'static [&'static str] = &[
        "SWAP_ALREADY_FULFILLED",
        "SWAP_ALREADY_FROZEN",
        "SWAP_ALREADY_VOIDED",
        "SWAP_ALREADY_CLOSED",
        "INVALID_FREEZING_SIGNATURE",
    ];
}

pub mod transfer {
    use crate::http::common::{SenderActiveState, Signature, Wallet};
    use crate::utils::{DateTime, UUID};

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
        pub tx_id: UUID,
        pub wallet: Wallet,
        pub wallet_balance: u64,
        pub wallet_balance_signature: Signature,
        pub wallet_signature: Signature,
    }

    pub const TRANSFER_CREATE_ERROR_CODES: &'static [&'static str] = &[
        "INVALID_DEBIT_AMOUNT",
        "CREDIT_WALLET_NOT_ADMITTED",
        "DEBIT_WALLET_NOT_ADMITTED",
        "DEBIT_CREDIT_WALLET_ADDRESS_MATCH",
        "EON_NUMBER_OUT_OF_SYNC",
        "DEBIT_WALLET_EXCEEDED_SLA",
        "CREDIT_WALLET_EXCEEDED_SLA",
        "DEBIT_WALLET_CANNOT_ADD_TRANSACTION",
        "CREDIT_WALLET_CANNOT_ADD_TRANSACTION",
        "DEBIT_WALLET_OVERSPENDING",
        "DEBIT_WALLET_BALANCE_MARKER_EXCEED_BALANCE",
        "INVALID_DEBIT_BALANCE_SIGNATURE",
        "INVALID_DEBIT_SIGNATURE",
    ];

    pub struct TransferReceipt {
        pub recipient: Wallet,
        pub recipient_active_state: SenderActiveState,
        pub sender_active_state: SenderActiveState,
        pub wallet: Wallet,
        pub wallet_signature: Signature,
    }

    pub const TRANSFER_UPDATE_ERROR_CODES: &'static [&'static str] = &[
        "TRANSFER_ALREADY_APPROVED",
        "TRANSFER_TIMED_OUT",
        "CREDIT_WALLET_EXCEEDED_SLA",
        "CREDIT_WALLET_CANNOT_ADD_TRANSACTION",
        "INVALID_CREDIT_SIGNATURE",
    ];
}
