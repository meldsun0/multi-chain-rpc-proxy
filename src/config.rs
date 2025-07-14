
use std::fs;
use serde::Deserialize;
use anyhow::{Context, Result};
use crate::model::blockchain::Blockchain;
use crate::model::provider::Provider;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
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

impl Config {
    pub fn load_from_file(path: &str) -> Result<Config> {
        let content = fs::read_to_string(path).with_context(|| format!("Failed to read config file: {}", path))?;
        let config: Config = serde_yaml::from_str(&content).with_context(|| format!("Failed to parse config YAML: {}", path))?;
        println!("{:#?}", config);
        Ok(config)
    }
}

impl From<&BlockchainConfig> for Blockchain {
    fn from(cfg: &BlockchainConfig) -> Self {
        let mut blockchain = Blockchain::new(cfg.id.clone(), cfg.name.clone());
        cfg.providers.iter().map(Provider::from).for_each(|p| blockchain.add_provider(p));
        blockchain
    }
}

impl From<&ProviderConfig> for Provider {
    fn from(p: &ProviderConfig) -> Self {
        Provider::new(p.id.clone(), p.name.clone(), p.host.clone(), p.port, p.path.clone(), p.priority, p.default)
    }
}