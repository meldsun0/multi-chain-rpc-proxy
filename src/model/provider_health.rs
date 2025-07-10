use getset::{Getters};
use std::time::SystemTime;
#[derive(Debug, Clone, Getters)]
pub struct Health {
    #[get_copy = "pub"]
    latency: u64,
    #[get_copy = "pub"]
    last_block: u64,
    #[get = "pub"]
    last_updated: SystemTime,
}

impl Health {
    pub fn new(latency: u64, last_block: u64) -> Self {
        Self {
            latency,
            last_block,
            last_updated: SystemTime::now(),
        }
    }
}