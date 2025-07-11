use std::error::Error;
use std::fs;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub blockchains: Vec<BlockchainConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BlockchainConfig {
    pub id: String,
    pub name: String,
    pub providers: Vec<ProviderConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProviderConfig {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub priority: u8,
    pub default: bool
}

impl AppConfig {
    pub fn load_from_file(path: &str) -> Result<AppConfig, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        let config: AppConfig = serde_yaml::from_str(&content)?;
        println!("{:#?}", config);
        Ok(config)
    }
}