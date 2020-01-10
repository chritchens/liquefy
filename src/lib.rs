//! `libliquefy` is the library used by the liquefy Liquidity Network hub client.
//! The library data models, requests and responses are built around those documented
//! in https://public.liquidity.network/swagger, https://rinkeby.liquidity.network/swagger
//! and https://limbo.liquidity.network/swagger.

/// Defines the errors used throughout the library.
pub mod error;

/// Defines the models used throughout the library.
pub mod model;

/// Defines the types and functions for the WS and HTTP Liquidity Network hub APIs.
pub mod request;
