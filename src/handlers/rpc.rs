use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use tracing::info;
use crate::model::rpc_proxy::RPCProxy;
use crate::services::request_forwarder::{ForwarderResponse};

#[post("/rpc")]pub async fn rpc(
    req_body: String,
    req: HttpRequest,
    state: web::Data<RPCProxy>
) -> impl Responder {
    info!("Received rpc request");
    let chain_id = match req.headers().get("X-Chain-Id").and_then(|v| v.to_str().ok()) {
        Some(id) if !id.is_empty() => id,
        _ => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Missing 'X-Chain-Id' HTTP header"
            }));
        }
    };

    let url = match state.get_best_provider_url(chain_id) {
        Some(url) => url,
        None => return HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("No RPC configured for chain ID '{}'", chain_id)
        })),
    };

    match state.forward(url, req_body).await {
        ForwarderResponse::Success { status, body } => {
            HttpResponse::build(actix_web::http::StatusCode::from_u16(status).unwrap_or_default())
                .content_type("application/json")
                .body(body.to_string())
        }
        ForwarderResponse::Error(err) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("RPC forward error: {}", err)
            }))
        }
    }
}