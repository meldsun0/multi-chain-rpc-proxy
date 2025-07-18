use async_trait::async_trait;
use reqwest::{Client, Response};
use serde_json::Value;

#[async_trait]
pub trait HttpForwarder: Send + Sync {
    async fn forward(&self, url: String, body: String) -> ForwarderResponse;
}

pub enum ForwarderResponse {
    Success {
        status: u16,
        body: Value,
    },
    Error(String),
}

#[derive(Clone)]
pub struct RequestForwarder {
    client: Client,
}

impl RequestForwarder {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .build()
                .expect("Failed to build client")
        }
    }
}


#[async_trait]
impl HttpForwarder for RequestForwarder {
    async fn forward(&self, url: String, body: String) -> ForwarderResponse {
        let result = self.client
            .post(url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await;

        match result {
            Ok(resp) => {
                let status = resp.status().as_u16();
                let bytes_result = resp.bytes().await;

                match bytes_result {
                    Ok(bytes) => {
                        match serde_json::from_slice::<Value>(&bytes) {
                            Ok(json) => ForwarderResponse::Success { status, body: json },
                            Err(err) => ForwarderResponse::Error(format!("Invalid JSON: {err}")),
                        }
                    }
                    Err(err) => ForwarderResponse::Error(format!("Failed to read response body: {err}")),
                }
            }
            Err(err) => ForwarderResponse::Error(format!("HTTP error: {err}")),
        }
    }
}
