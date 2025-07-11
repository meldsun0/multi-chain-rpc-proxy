use std::sync::Arc;
use actix_web::{web, App, HttpServer};

use crate::model::AppState;
use crate::config::AppConfig;

mod endpoints;
pub mod provider_querier;

pub async fn run() -> std::io::Result<()> {
    let config = match AppConfig::load_from_file("config.yaml") {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load config file: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)));
        }
    };

    let state = Arc::new(AppState::new(&config));

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