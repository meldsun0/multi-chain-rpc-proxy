use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Default)]
pub struct AppState {
    pub default_chain: String,
}

impl AppState {
    pub fn get_provider(&self, _chain_id: &str) -> Option<&String> {
       // self.rpc_urls.get(chain_id)
        None
    }

    pub fn get_metrics(&self) -> Option<String> {
        None
    }
}