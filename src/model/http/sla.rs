//! `sla` defines the models for interacting with
//! the HTTP SLA (Service Level Agreement) API.

use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::model::common::{DateTime, Decimal};
use crate::model::traits::{FromJson, ToJson};
use crate::result::Result;

/// `SLA` represents a Service Level Agreement.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SLA {
    pub cost: Decimal,
    pub limit: u64,
    pub recipient: String,
    pub token: String,
}

impl SLA {
    /// `MIN_LIMIT` is the minimum value of a limit field.
    pub const MIN_LIMIT: u64 = 1;

    /// `RECIPIENT_LENGTH` is the length of a recipient field.
    pub const RECIPIENT_LENGTH: usize = 42;

    /// `TOKEN_LENGTH` is the length of a token field.
    pub const TOKEN_LENGTH: usize = 42;

    /// `new` creates a new `SLA`.
    pub fn new(cost: Decimal, limit: u64, recipient: &str, token: &str) -> Result<SLA> {
        if limit < Self::MIN_LIMIT {
            return Err(Error::OutOfRange {
                value: limit as i64,
                min: Some(Self::MIN_LIMIT as i64),
                max: None,
            });
        }

        if recipient.len() != Self::RECIPIENT_LENGTH {
            return Err(Error::InvalidLength {
                length: recipient.len(),
                expected: Self::RECIPIENT_LENGTH,
            });
        }

        if token.len() != Self::TOKEN_LENGTH {
            return Err(Error::InvalidLength {
                length: token.len(),
                expected: Self::TOKEN_LENGTH,
            });
        }

        let sla = SLA {
            cost,
            limit,
            recipient: recipient.to_owned(),
            token: token.to_owned(),
        };

        Ok(sla)
    }

    /// `validate` validates the `SLA`.
    pub fn validate(&self) -> Result<()> {
        if self.limit < Self::MIN_LIMIT {
            return Err(Error::OutOfRange {
                value: self.limit as i64,
                min: Some(Self::MIN_LIMIT as i64),
                max: None,
            });
        }

        if self.recipient.len() != Self::RECIPIENT_LENGTH {
            return Err(Error::InvalidLength {
                length: self.recipient.len(),
                expected: Self::RECIPIENT_LENGTH,
            });
        }

        if self.token.len() != Self::TOKEN_LENGTH {
            return Err(Error::InvalidLength {
                length: self.token.len(),
                expected: Self::TOKEN_LENGTH,
            });
        }

        Ok(())
    }
}

impl ToJson for SLA {}

impl<'a> FromJson<'a> for SLA {}

/// `WalletSLA` is a Wallet SLA
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WalletSLA {
    pub expiry: DateTime,
}

impl WalletSLA {
    /// `new` creates a new `WalletSLA`.
    pub fn new(expiry: DateTime) -> WalletSLA {
        WalletSLA { expiry }
    }
}

impl ToJson for WalletSLA {}

impl<'a> FromJson<'a> for WalletSLA {}
