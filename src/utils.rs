//! `utils` contains the utility types and functions used throughout libliquefy.

use chrono::Utc;
use uriparse;
use uuid::Uuid;

/// `DateTime` is a UTC datetime.
pub struct DateTime(chrono::DateTime<Utc>);

/// `UUID` is a v4 UUID.
pub struct UUID(Uuid);

/// `URI` is an RFC3986 URI.
pub struct URI<'a>(uriparse::URI<'a>);
