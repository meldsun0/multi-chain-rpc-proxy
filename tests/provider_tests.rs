use rpc_proxy_meldsun0::model::provider::{Provider, ProviderError};

#[test]
fn test_valid_provider_creation() {
    let provider = Provider::new(
        "p1",
        "Provider One",
        "localhost",
        8080,
        "/health",
        5,
        true,
    ).unwrap();

    assert_eq!(provider.id(), "p1");
    assert_eq!(provider.name(), "Provider One");
    assert_eq!(provider.host(), "localhost");
    assert_eq!(*provider.port(), 8080);
    assert_eq!(*provider.priority(), 5);
    assert_eq!(*provider.default(), true);
    assert_eq!(provider.get_health_endpoint_url(), "https://localhost:8080/health");
}

#[test]
fn test_empty_id_returns_error() {
    let result = Provider::new(
        "",
        "Provider",
        "localhost",
        8080,
        "/health",
        5,
        false,
    );
    assert!(matches!(result, Err(ProviderError::EmptyId)));
}

#[test]
fn test_empty_host_returns_error() {
    let result = Provider::new(
        "p1",
        "Provider",
        "   ",
        8080,
        "/health",
        5,
        false,
    );
    assert!(matches!(result, Err(ProviderError::EmptyHost)));
}

#[test]
fn test_invalid_port_returns_error() {
    let result = Provider::new(
        "p1",
        "Provider",
        "localhost",
        0,
        "/health",
        5,
        false,
    );
    assert!(matches!(result, Err(ProviderError::InvalidPort(0))));
}

#[test]
fn test_invalid_priority_returns_error() {
    let result = Provider::new(
        "p1",
        "Provider",
        "localhost",
        8080,
        "/health",
        200,
        false,
    );
    assert!(matches!(result, Err(ProviderError::InvalidPriority(200))));
}

#[test]
fn test_empty_health_path_returns_error() {
    let result = Provider::new(
        "id",
        "Provider",
        "localhost",
        8080,
        "",
        5,
        false,
    );
    assert!(matches!(result, Err(ProviderError::EmptyHealthPath)));
}
