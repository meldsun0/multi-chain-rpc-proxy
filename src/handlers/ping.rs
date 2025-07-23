use actix_web::{get, HttpResponse, Responder};
use tracing::info;

#[get("/")]
pub async fn ping() -> impl Responder {
    info!("Received ping request");
    HttpResponse::Ok().body("Multi-Chain RPC Proxy up!".to_string())
}