use actix_web::{get, web, HttpResponse, Responder};
use tracing::info;
use crate::model::rpc_proxy::RPCProxy;

#[get("/metrics")]
pub async fn metrics(rpc_proxy: web::Data<RPCProxy>) -> impl Responder {
    info!("Received metrics request");
    match rpc_proxy.get_metrics() {
        Ok(metrics) => HttpResponse::Ok().json(metrics),
        Err(err) => {
            log::error!("Failed to get metrics: {}", err);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to generate metrics"
            }))
        }
    }
}

