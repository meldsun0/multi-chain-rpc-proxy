use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Multi-Chain RPC Proxy up!".to_string())
}