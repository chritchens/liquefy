//! `utils` contains the utility types and functions used throughout libliquefy.

use uriparse;
use uuid::Uuid;

/// `UUID` is a UUID value of no specified version.
pub struct UUID(Uuid);

/// `URI` is an RFC3986 URI.
pub struct URI<'a>(uriparse::URI<'a>);
