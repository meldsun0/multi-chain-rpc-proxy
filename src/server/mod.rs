use std::sync::Arc;
use actix_web::{web, App, HttpServer};

use crate::endpoints;
use crate::model::AppState;
use crate::config::AppConfig;

pub async fn run() -> std::io::Result<()> {
    let config = AppConfig::load();
    let state = Arc::new(AppState::new());
    state.init_health_checker();

    let actix_state = web::Data::from(state);

    HttpServer::new(move || {
        App::new()
            .app_data(actix_state.clone())
            .service(endpoints::metrics)
            .service(endpoints::ping)
            .service(endpoints::handle_rpc)
    })
        .bind((config.host.as_str(), config.port))?
        .run()
        .await
}