use getset::{Getters};
use std::time::SystemTime;
use thiserror::Error;

#[derive(Debug, Clone, Getters)]
pub struct Health {
    #[get = "pub"]
    latency: u64,
    #[get = "pub"]
    last_block: u64,
    #[get = "pub"]
    last_updated: SystemTime,
}

#[derive(Debug, Error)]
pub enum HealthError {
    #[error("Invalid health data: latency={latency}, last_block={last_block}")]
    InvalidData { latency: u64, last_block: u64 },
}

impl Health {
    pub fn new(latency: u64, last_block: u64) -> Result<Self, HealthError> {
        if latency == 0 || last_block == 0 {
            return Err(HealthError::InvalidData { latency, last_block });
        }
        Ok(Self {
            latency,
            last_block,
            last_updated: SystemTime::now(),
        })
    }
}