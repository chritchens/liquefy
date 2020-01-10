//! `analytics` defines the models for interacting with
//! the HTTP Analytics API.

use serde::{Deserialize, Serialize};

use crate::model::common::{Block, DateTime};

/// `Snapshot` is an eon snapshot.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct EonSnapshot {
    pub count: u64,
    pub eon_number: u64,
}

/// `DaySnapshot` is the snapshot of a day.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DaySnapshot {
    pub count: u64,
    pub day: DateTime,
}

/// `Challenges` is the aggregated information of challenges.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Challenges {
    pub eon_number: Vec<EonSnapshot>,
    pub rebuted: u64,
    pub time: Vec<DaySnapshot>,
}

/// `Deposits` is the aggregated information of deposits.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Deposits {
    pub eon_number: Vec<EonSnapshot>,
    pub time: Vec<DaySnapshot>,
    pub total: u64,
}

/// `OperatorStatus` is the status of an operator.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct OperatorStatus {
    pub blocks_per_eon: u64,
    pub confirmed: Vec<Block>,
    pub current_eon_number: u64,
    pub latest: Block,
}

/// `Transfers` is the aggregated information of transfers.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Transfers {
    pub eon_number: Vec<EonSnapshot>,
    pub time: Vec<DaySnapshot>,
    pub total: u64,
}

/// `Admissions` is the aggregated information of admissions.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Admissions {
    pub eon_number: Vec<EonSnapshot>,
    pub total: u64,
}

/// `Withdrawals` represents withdrawals information.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Withdrawals {
    pub eon_number: Vec<EonSnapshot>,
    pub time: Vec<DaySnapshot>,
    pub total: u64,
}
