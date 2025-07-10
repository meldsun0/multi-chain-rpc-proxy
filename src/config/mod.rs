#[derive(Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Self {
        AppConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        }
    }
}