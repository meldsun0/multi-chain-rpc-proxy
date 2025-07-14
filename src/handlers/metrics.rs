use actix_web::{get, web, HttpResponse, Responder};
use crate::model::rpc_proxy::RPCProxy;

#[get("/metrics")]
pub async fn metrics(rpc_proxy: web::Data<RPCProxy>) -> impl Responder {
    match rpc_proxy.get_metrics() {
        Some(response) => HttpResponse::Ok().json(response),
        None => HttpResponse::Ok().json(serde_json::json!({
            "message": "No metrics available"
        })),
    }
}

