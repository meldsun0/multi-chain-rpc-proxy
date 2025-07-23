use std::env::args;
use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use rpc_proxy_meldsun0::config::Config;
use anyhow::{Result};
use tracing::info;
use rpc_proxy_meldsun0::handlers::metrics::metrics;
use rpc_proxy_meldsun0::handlers::ping::ping;
use rpc_proxy_meldsun0::handlers::rpc::rpc;
use rpc_proxy_meldsun0::logger::Logger;
use rpc_proxy_meldsun0::model::rpc_proxy::RPCProxy;
use rpc_proxy_meldsun0::runtime_mode::{DevModeForwarder, DevModeProvider};
use rpc_proxy_meldsun0::services::default_provider_client::DefaultProviderQuerier;
use rpc_proxy_meldsun0::services::request_forwarder::RequestForwarder;

#[actix_web::main]
async fn main() -> Result<()> {
    Logger::init();
    let args: Vec<String> = args().collect();
    info!("Starting Multi-Chain RPC proxy {}", if args.contains(&"--dev".to_string()) { " in DEV mode" } else { "" });
    let config = Config::load_from_file("config.yaml")?;

    let rpc_proxy = init_rpc_proxy(&args, &config)?;

    let actix_state = web::Data::from( Arc::new(rpc_proxy));

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

fn init_rpc_proxy(args: &[String], config: &Config) -> Result<RPCProxy> {
    if args.contains(&"--dev".to_string()) {
        let proxy = RPCProxy::init(config, Arc::new(DevModeForwarder::new()))?;
        proxy.init_health_checker(DevModeProvider::new());
        Ok(proxy)
    } else {
        info!("Starting Multi-Chain RPC proxy");
        let proxy = RPCProxy::init(config, Arc::new(RequestForwarder::new()))?;
        proxy.init_health_checker(DefaultProviderQuerier::new(3));
        Ok(proxy)
    }
}



