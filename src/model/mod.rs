//! Core data models used throughout the application.
//!
//! Includes types for representing blockchains, providers, health status, and metrics.
//! These models are typically shared across handlers, services, and configuration.
pub mod provider;
pub mod blockchain;
pub mod provider_health;
pub mod metrics;
pub mod provider_ranker;
pub mod rpc_proxy;


