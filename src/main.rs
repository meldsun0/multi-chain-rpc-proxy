use std::sync::Arc;
use actix_web::{web, App, HttpServer};


use rpc_proxy_meldsun0::config::Config;

use anyhow::Result;
use rpc_proxy_meldsun0::handlers::metrics::metrics;
use rpc_proxy_meldsun0::handlers::ping::ping;
use rpc_proxy_meldsun0::handlers::rpc::rpc;
use rpc_proxy_meldsun0::model::rpc_proxy::RPCProxy;
use rpc_proxy_meldsun0::services::default_provider_client::DefaultProviderQuerier;

#[actix_web::main]
async fn main() -> Result<()> {

    let config = Config::load_from_file("config.yaml")?;

    let rpc_proxy = Arc::new(RPCProxy::new(&config));


    rpc_proxy.init_health_checker(DefaultProviderQuerier::new(3));

    let actix_state = web::Data::from(rpc_proxy);

    HttpServer::new(move || {
        App::new()
            .app_data(actix_state.clone())
            .service(metrics)
            .service(ping)
            .service(rpc)
    })
        .bind((config.host, config.port))?
        .run()
        .await?;

    Ok(())
}