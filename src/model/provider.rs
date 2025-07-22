use getset::Getters;
use thiserror::Error;

#[derive(Debug, Clone, Getters)]
pub struct Provider {
    #[get = "pub"]
    id: String,

    #[get = "pub"]
    name: String,

    #[get = "pub"]
    host: String,

    #[get = "pub"]
    port: u16,

    #[get = "pub"]
    health_path : String,

    #[get = "pub"]
    priority: u8,

    #[get = "pub"]
    default: bool,
}

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("Provider ID cannot be empty")]
    EmptyId,

    #[error("Host cannot be empty")]
    EmptyHost,

    #[error("Invalid port number: {0}")]
    InvalidPort(u16),

    #[error("Priority must be between 0 and 100, got {0}")]
    InvalidPriority(u8),

    #[error("Health path cannot be empty")]
    EmptyHealthPath,
}

impl Provider {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        host: impl Into<String>,
        port: u16,
        health_path: impl Into<String>,
        priority: u8,
        default: bool,
    ) -> Result<Self, ProviderError> {
        let id = id.into();
        if id.trim().is_empty() {
            return Err(ProviderError::EmptyId);
        }

        let host = host.into();
        if host.trim().is_empty() {
            return Err(ProviderError::EmptyHost);
        }

        if port == 0 {
            return Err(ProviderError::InvalidPort(port));
        }

        if priority > 100 {
            return Err(ProviderError::InvalidPriority(priority));
        }
        let health_path = health_path.into();
        if health_path.trim().is_empty() {
            return Err(ProviderError::EmptyHealthPath);
        }

        Ok(Self {
            id,
            name: name.into(),
            host,
            port,
            health_path: health_path.into(),
            priority,
            default,
        })
    }

    pub fn get_health_endpoint_url(&self) -> String {
        format!("https://{}:{}{}", self.host, self.port, self.health_path)
    }
}

