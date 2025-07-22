use std::fs;
use serde::Deserialize;
use anyhow::{Context, Result};
use crate::model::blockchain::Blockchain;
use crate::model::provider::{Provider, ProviderError};

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
    pub default: bool,
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Config> {
        let content = fs::read_to_string(path).with_context(|| format!("Failed to read config file: {}", path))?;
        let config: Config = serde_yaml::from_str(&content).with_context(|| format!("Failed to parse config YAML: {}", path))?;
        println!("{:#?}", config);
        Ok(config)
    }
}

impl TryFrom<&BlockchainConfig> for Blockchain {
    type Error = anyhow::Error;
    fn try_from(cfg: &BlockchainConfig) -> Result<Self, Self::Error> {
        let mut blockchain = Blockchain::new(cfg.id.clone(), cfg.name.clone());

        for provider_cfg in &cfg.providers {
            let provider = Provider::try_from(provider_cfg).with_context(|| format!("Failed to convert provider '{}'", provider_cfg.id))?;
            blockchain.add_provider(provider);
        }
        Ok(blockchain)
    }
}

impl TryFrom<&ProviderConfig> for Provider {
    type Error = ProviderError;
    fn try_from(p: &ProviderConfig) -> Result<Self, Self::Error> {
        Provider::new(
            p.id.clone(),
            p.name.clone(),
            p.host.clone(),
            p.port,
            p.path.clone(),
            p.priority,
            p.default,
        )
    }
}