mod helpers;

use std::io::Write;
use tempfile::NamedTempFile;
use rpc_proxy_meldsun0::config::Config;

#[test]
fn test_load_from_valid_yaml_file() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");

    let yaml_content = r#"
host: "127.0.0.1"
port: 8080
blockchains:
  - id: "eth-mainnet"
    name: "Ethereum Mainnet"
    providers:
      - id: "alchemy"
        name: "Alchemy"
        host: "eth-mainnet.alchemyapi.io"
        port: 443
        path: "/v2/xyz"
        priority: 1
        default: true
"#;

    write!(temp_file, "{}", yaml_content).expect("Failed to write to temp file");

    let config = Config::load_from_file(temp_file.path().to_str().unwrap()).expect("Failed to load config");

    assert_eq!(config.host, "127.0.0.1");
    assert_eq!(config.port, 8080);
    assert_eq!(config.blockchains.len(), 1);
    assert_eq!(config.blockchains[0].id, "eth-mainnet");
    assert_eq!(config.blockchains[0].providers.len(), 1);
    assert_eq!(config.blockchains[0].providers[0].id, "alchemy");
}

