use getset::Getters;
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

    health_path : String,

    #[get = "pub"]
    priority: u8,

    #[get = "pub"]
    default: bool,
}

impl Provider {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        host: impl Into<String>,
        port: u16,
        health_path : impl Into<String>,
        priority: u8,
        default: bool,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            host: host.into(),
            port,
            health_path :  health_path .into(),
            priority,
            default
        }
    }
    pub fn get_health_endpoint_url(&self) -> String {
        format!("https://{}:{}{}", self.host, self.port, self.health_path)
    }
}

