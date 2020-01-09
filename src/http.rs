//! `http` contains the types and functions for the HTTP API of the Liquid Network hub.

use chrono::{DateTime, Utc};

pub mod admission {
    pub struct Admission {
        pub address: String, // NB: [1..40] chars
        pub authorization: String, // NB: [1..130] chars
        pub token: String, // NB: [1..42] chars
    }
}

pub mod analytics {
    pub struct EonSnapshot {
        pub count: u64,
        pub eon_number: u64,
    }

    pub struct DaySnapshot {
        pub count: u64,
        pub day: DateTime<Utc>, // NB: fetched as string
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

    pub struct ConfirmedBlock {
        pub block: u64,
        pub eon_number: u64,
    }

    pub struct OperatorStatus {
        pub blocks_per_eon: u64,
        pub confirmed: Vec<ConfirmedBlock>,
        pub current_eon_number: u64,
        pub latest: ConfirmedBlock,
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
    pub struct Blocks {
        pub confirmed: Vec<ConfirmedBlock>,
        pub latest: ConfirmedBlock,
    }

    pub struct SwapAudit {
        pub amount: f64, // NB: as string when fetched
        pub amount_swapped: f64, // NB: as string when fetched
        pub remaining_in: f64, // NB: as string when fetched
        pub remaining_out: f64, // NB: as string when fetched
    }

    pub struct TokenOrderBook {
        pub buy_orders: Vec<SwapAudit>,
        pub sell_orders: Vec<SwapAudit>,
    }

    // NB: only whitelisted IPs can access this data
    pub struct TokenMatching {
        pub price: f64, // NB: as string when fetched
        pub time: u64,
        pub volume: f64, // NB: as string when fetched
    }

    pub struct Token {
        pub address: String, // NB: [1..40] chars
        pub name: Option<String>, // NB: <= 40 chars
        pub short_name: Option<String>, // NB: <= 40 chars
    }

    pub struct Transfers {
        // TODO
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
    pub struct SLA {
        pub cost: f64, // NB: as string when fetched
        pub limit: u64, // NB: limit >= 1
        pub recipient: String, // NB: [1..42] chars
        pub token: String, // NB: [1..42] chars
    }

    pub struct WalletSLA {
        pub expiry: DateTime<Utc>, // NB: fetched as string
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
