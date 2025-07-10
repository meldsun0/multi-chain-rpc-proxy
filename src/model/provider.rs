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

    #[get = "pub"]
    priority: u8,

    #[get = "pub"]
    default: bool,

    health_path : String,

}

impl Provider {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        host: impl Into<String>,
        port: u16,
        priority: u8,
        health_path : impl Into<String>,
        default: bool,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            host: host.into(),
            port,
            priority,
            health_path :  health_path .into(),
            default
        }
    }
    pub fn get_health_endpoint_url(&self) -> String {
        format!("https://{}:{}{}", self.host, self.port, self.health_path)
    }

}

