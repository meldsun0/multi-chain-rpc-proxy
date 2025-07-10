use actix_web::{post, web, HttpResponse, HttpRequest, Responder, get};
use reqwest::Client;
use crate::state::AppState;

#[get("/")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Multi-Chain RPC Proxy up!".to_string())
}

#[get("/metrics")]
pub async fn metrics(state: web::Data<AppState>) -> impl Responder {
    match state.get_metrics() {
        Some(response) => HttpResponse::Ok().body(response),
        None => HttpResponse::InternalServerError().json(serde_json::json!({
        "error": "No metrics available"
    })),
    }
}


#[post("/rpc")]
pub async fn handle_rpc(
    req_body: String,
    req: HttpRequest,
    state: web::Data<AppState>,
) -> impl Responder {

    let chain_id = req
        .headers()
        .get("X-Chain-Id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or(&state.default_chain);


    let url = match state.get_provider(chain_id) {
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