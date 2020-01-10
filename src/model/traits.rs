//! `traits` defines traits used in the module.

use serde::{Deserialize, Serialize};
use serde_json as json;

use crate::result::Result;

/// `ToJson` is the trait implemented by types that can be serialized to JSON.
pub trait ToJson: Serialize {
    /// `to_json` serializes the object into a json value.
    fn to_json(&self) -> Result<String> {
        json::to_string(self).map_err(|e| e.into())
    }
}

pub trait FromJson<'a>: Deserialize<'a> {
    // `from_json` deserializes an object from a json value.
    fn from_json(s: &'a str) -> Result<Self> {
        json::from_str(s).map_err(|e| e.into())
    }
}
