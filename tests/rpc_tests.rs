use std::sync::Arc;
use actix_web::{test, web, App};
use actix_web::http::StatusCode;
use rpc_proxy_meldsun0::config::Config;
use rpc_proxy_meldsun0::handlers::rpc::rpc;
use rpc_proxy_meldsun0::model::rpc_proxy::RPCProxy;
use crate::helpers::mock_forwarder::MockForwarder;

mod helpers;

#[actix_web::test]
async fn test_missing_chain_id_header() {
    let config = Config::load_from_file("tests/fixture/config.test.yaml").unwrap();
    let rpc_proxy =RPCProxy::init(&config, Arc::new(MockForwarder::new())).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from( Arc::new(rpc_proxy)))
            .service(rpc)
    ).await;

    let req = test::TestRequest::post().uri("/rpc").set_payload("{}").to_request();
    let resp: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp["error"], "Missing 'X-Chain-Id' HTTP header");

}

#[actix_web::test]
async fn test_empty_chain_id_header() {
    let config = Config::load_from_file("tests/fixture/config.test.yaml").unwrap();
    let rpc_proxy =  RPCProxy::init(&config, Arc::new(MockForwarder::new())).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(Arc::new(rpc_proxy)))
            .service(rpc)
    ).await;

    let req = test::TestRequest::post()
        .uri("/rpc")
        .insert_header(("X-Chain-Id", ""))
        .set_payload("{}")
        .to_request();

    let resp: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(resp["error"], "Missing 'X-Chain-Id' HTTP header");
}


#[actix_web::test]
async fn test_no_provider_for_chain_id() {
    let config = Config::load_from_file("tests/fixture/config.test.yaml").unwrap();
    let rpc_proxy =  RPCProxy::init(&config, Arc::new(MockForwarder::new())).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(Arc::new(rpc_proxy)))
            .service(rpc)
    ).await;

    let req = test::TestRequest::post()
        .uri("/rpc")
        .insert_header(("X-Chain-Id", "xzy"))
        .set_payload("{}")
        .to_request();

    let resp: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(resp["error"], "No RPC configured for chain ID 'xzy'");
}

#[actix_web::test]
async fn test_successful_response() {
    let config = Config::load_from_file("tests/fixture/config.test.yaml").unwrap();
    let rpc_proxy =  RPCProxy::init(&config, Arc::new(MockForwarder::new())).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(Arc::new(rpc_proxy)))
            .service(rpc)
    ).await;

    let req = test::TestRequest::post()
        .uri("/rpc")
        .insert_header(("X-Chain-Id", "eth"))
        .set_payload("{}")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

