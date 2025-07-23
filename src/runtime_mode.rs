
use rand::Rng;
use async_trait::async_trait;
use serde_json::json;
use derive_new::new;
use crate::services::default_provider_client::{ProviderQueryError, ProviderResponse};
use crate::services::health_checker::ProviderQuery;
use crate::services::request_forwarder::{ForwarderResponse, HttpForwarder, RequestForwarder};

#[derive(Clone, new)]
pub struct DevModeForwarder;

#[derive(Clone, new)]
pub struct DevModeProvider;

#[async_trait]
impl ProviderQuery for DevModeProvider {
    async fn query(&self, _url: &str) -> Result<ProviderResponse, ProviderQueryError> {
        let mut rng = rand::thread_rng();
        let latency_ms = rng.gen_range(50..500);
        let last_block = rng.gen_range(10_000_000..20_000_000);
        Ok(ProviderResponse::new(latency_ms, last_block))
    }
}

#[async_trait]
impl HttpForwarder for DevModeForwarder {
    async fn forward(&self, url: String, body: String)  -> ForwarderResponse {
        if url.contains("fail") || body.contains("fail") {
            ForwarderResponse::Error("Simulated forwarding failure".to_string())
        } else {
            ForwarderResponse::Success {
                status: 200,
                body: json!({
                    "mock": true,
                    "url": url,
                    "body": body
                }),
            }
        }
    }
}