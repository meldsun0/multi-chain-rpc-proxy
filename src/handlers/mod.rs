//! HTTP route handlers for the Actix Web server.
//!
//! This module defines the endpoints served by the application.
//!
//! Includes:
//! - `metrics`:metrics output.
//! - `ping`: Simple health check endpoint.
//! - `rpc`: Proxy logic for forwarding JSON-RPC requests.
pub mod ping;
pub mod rpc;
pub  mod metrics;