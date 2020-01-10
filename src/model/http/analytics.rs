//! `analytics` defines the models for interacting with
//! the HTTP Analytics API.

use crate::model::common::{Block, DateTime};

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
