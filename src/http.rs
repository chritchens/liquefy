//! `http` contains the types and functions for the HTTP API of the Liquid Network hub.

pub mod error {
    use std::collections::HashMap;

    pub struct ErrorData {
        pub code: Vec<String>,
        pub message: String,
    }

    pub struct ErrorResponse {
        pub non_field_errors: Vec<ErrorData>,
        pub others: HashMap<String, String>,
    }
}

pub mod common {
    pub struct Block {
        pub block: u64,
        pub eon_number: u64,
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
    use chrono::{DateTime, Utc};

    use crate::http::common::Block;

    pub struct EonSnapshot {
        pub count: u64,
        pub eon_number: u64,
    }

    pub struct DaySnapshot {
        pub count: u64,
        pub day: DateTime<Utc>,
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
    use crate::http::common::Block;
    use crate::utils::URI;

    pub struct Blocks {
        pub confirmed: Vec<Block>,
        pub latest: Block,
    }

    pub struct SwapAudit {
        pub amount: f64,
        pub amount_swapped: f64,
        pub remaining_in: f64,
        pub remaining_out: f64,
    }

    pub struct TokenOrderBook {
        pub buy_orders: Vec<SwapAudit>,
        pub sell_orders: Vec<SwapAudit>,
    }

    pub struct TokenMatching {
        pub price: f64,
        pub time: u64,
        pub volume: f64,
    }

    pub struct Token {
        pub address: String,
        pub name: Option<String>,
        pub short_name: Option<String>,
    }

    pub struct ConciseTransfer {
        pub amount: f64,
        pub amount_swapped: f64,
        pub cancelled: bool,
        pub complete: bool,
        pub eon_number: i64,
        pub id: i64,
        pub nonce: u64,
        pub passive: bool,
        // TODO
    }

    pub struct Transfers<'a> {
        pub count: i64,
        pub next: Option<URI<'a>>,
        pub previous: Option<URI<'a>>,
        pub results: Vec<ConciseTransfer>,
    }

    pub struct Transfer {
        // TODO
    }

    pub struct WalletState {
        // TODO
    }

    pub struct WhoIs {
        // TODO
    }
}

pub mod sla {
    use chrono::{DateTime, Utc};

    pub struct SLA {
        pub cost: f64,
        pub limit: u64,
        pub recipient: String,
        pub token: String,
    }

    pub struct WalletSLA {
        pub expiry: DateTime<Utc>,
    }
}

pub mod swap {
    pub struct Swap {
        // TODO
    }

    pub struct SwapCancellation {
        // TODO
    }

    pub struct SwapFinalization {
        // TODO
    }
}

pub mod transfer {
    pub struct Transfer {
        // TODO
    }
}
