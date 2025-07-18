use async_trait::async_trait;
use derive_new::new;
use serde_json::json;
use rpc_proxy_meldsun0::services::request_forwarder::{ForwarderResponse, HttpForwarder};

#[derive(Clone, new)]
pub struct MockForwarder;

#[async_trait]
impl HttpForwarder for MockForwarder {
    async fn forward(&self, url: String, body: String) -> ForwarderResponse {

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