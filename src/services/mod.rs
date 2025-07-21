//! Business logic and background services.
//!
//! This module provides service abstractions such as:
//! - Provider health checking
//! - Default provider selection
//! - Request forwarding
//! - Metrics generation
pub mod default_provider_client;
pub mod health_checker;
pub mod metrics_generator;
pub mod request_forwarder;