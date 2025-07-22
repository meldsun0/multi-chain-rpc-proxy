use actix_web::web::Bytes;
use std::sync::Arc;
use actix_web::{test, web, App};
use rpc_proxy_meldsun0::config::Config;
use rpc_proxy_meldsun0::handlers::ping::ping;
use rpc_proxy_meldsun0::model::rpc_proxy::RPCProxy;
use crate::helpers::mock_forwarder::MockForwarder;

mod helpers;

#[actix_web::test]
async fn test_ping_endpoint() {
    let config = Config::load_from_file("tests/fixture/config.test.yaml").unwrap();
    let rpc_proxy = RPCProxy::init(&config,  Arc::new(MockForwarder::new())).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(Arc::new(rpc_proxy)))
            .service(ping)
    ).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let body: Bytes = test::call_and_read_body(&app, req).await;
    assert_eq!(body, "Multi-Chain RPC Proxy up!");

}

