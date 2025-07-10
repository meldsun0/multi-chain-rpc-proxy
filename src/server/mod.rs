use actix_web::{web, App, HttpServer};

use crate::endpoints;
use crate::model::AppState;
use crate::config::AppConfig;

pub async fn run() -> std::io::Result<()> {
    let config = AppConfig::load();
    let state = web::Data::new(AppState::default());

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(endpoints::metrics)
            .service(endpoints::ping)
            .service(endpoints::handle_rpc)
    })
        .bind((config.host.as_str(), config.port))?
        .run()
        .await
}