//! `libliquefy` is the library used by the liquefy Liquid Network hub client.
//! The library data models, requests and responses are built around those documented
//! in https://public.liquid.network/swagger, https://rinkeby.liquid.network/swagger
//! and https://limbo.liquid.network/swagger.

/// Defines utilities used throughout the library.
pub mod utils;

/// Defines the types and functions for the WebSocket Liquidity Network hub API.
pub mod ws;

/// Defines the types and functions for the HTTP Liquidity Network hub API.
pub mod http;
