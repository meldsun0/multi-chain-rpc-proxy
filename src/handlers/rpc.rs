use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use reqwest::Client;
use crate::model::rpc_proxy::RPCProxy;

#[post("/rpc")]
pub async fn rpc(
    req_body: String,
    req: HttpRequest,
    state: web::Data<RPCProxy>,
) -> impl Responder {

    let chain_id = match req.headers().get("X-Chain-Id").and_then(|v| v.to_str().ok()) {
        Some(id) if !id.is_empty() => id,
        _ => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Missing or invalid 'X-Chain-Id' header"
            }));
        }
    };

    let url = match state.get_best_provider_url(chain_id) {
        Some(url) => url,
        None => return HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("No RPC configured for chain ID '{}'", chain_id)
        })),
    };


    let client = Client::new();
    let result = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(req_body)
        .send()
        .await;

    match result {
        Ok(response) => {
            let status = response.status();
            let body = response.bytes().await.unwrap_or_default();
            HttpResponse::build(status).body(body)
        }
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("RPC forward error: {}", err)
        })),
    }
}