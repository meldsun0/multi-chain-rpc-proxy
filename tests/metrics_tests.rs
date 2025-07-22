mod helpers;

use std::sync::Arc;
use std::time::Duration;
use actix_web::{test, web, App};
use actix_web::http::StatusCode;
use rpc_proxy_meldsun0::config::Config;
use rpc_proxy_meldsun0::handlers::metrics::metrics;

use rpc_proxy_meldsun0::model::rpc_proxy::RPCProxy;
use crate::helpers::mock_forwarder::MockForwarder;
use crate::helpers::MockQuerier;

#[actix_web::test]
async fn test_metrics_endpoint() {
    let config = Config::load_from_file("tests/fixture/config.test.yaml").unwrap();
    let rpc_proxy =  RPCProxy::init(&config, Arc::new(MockForwarder::new())).unwrap();

    rpc_proxy.init_health_checker(MockQuerier);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(Arc::new(rpc_proxy)))
            .service(metrics)
    ).await;

    tokio::time::sleep(Duration::from_secs(3)).await;

    let req = test::TestRequest::get().uri("/metrics").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;

    for blockchain in body.get("blockchains").unwrap().as_array().unwrap() {
        let blockchain_id = blockchain.get("id").unwrap().as_str().unwrap();
        let providers = blockchain.get("providers").unwrap().as_array().unwrap();

        for provider in providers {
            let id = provider.get("id").unwrap().as_str().unwrap();
            let name = provider.get("name").unwrap().as_str().unwrap();
            let priority = provider.get("priority").unwrap().as_u64().unwrap();
            let is_default = provider.get("default").unwrap().as_bool().unwrap();

            let health = provider.get("health").unwrap().as_object().unwrap();
            let latency_ms = health.get("latency_ms").unwrap().as_u64().unwrap();
            let last_block = health.get("last_block").unwrap().as_u64().unwrap();

            let rank = provider.get("rank").unwrap().as_object().unwrap();
            let score = rank.get("score").unwrap().as_u64().unwrap();

            match id {
                "op-p1" => {
                    assert_eq!(name, "Optimism Provider1");
                    assert_eq!(priority, 2);
                    assert!(is_default);
                    assert_eq!(blockchain_id, "op");
                }
                "op-p2" => {
                    assert_eq!(name, "Optimism Provider2");
                    assert_eq!(priority, 1);
                    assert!(!is_default);
                    assert_eq!(blockchain_id, "op");
                }
                "eth-p1" => {
                    assert_eq!(name, "Ethereum Provider1");
                    assert_eq!(priority, 1);
                    assert!(!is_default);
                    assert_eq!(blockchain_id, "eth");
                }
                "eht-p2" => {
                    assert_eq!(name, "Ethereum Provider2");
                    assert_eq!(priority, 20);
                    assert!(is_default);
                    assert_eq!(blockchain_id, "eth");
                }
                "eht-p3" => {
                    assert_eq!(name, "Ethereum Provider3");
                    assert_eq!(priority, 7);
                    assert!(is_default);
                    assert_eq!(blockchain_id, "eth");
                }
                _ => panic!("Unexpected provider id: {}", id),
            }
            assert!(score > 0, "score should be > 0");
            assert!(latency_ms > 0, "latency_ms too low: {}", latency_ms);
            assert!(last_block > 0, "last_block too low: {}", last_block);
        }
    }
}

