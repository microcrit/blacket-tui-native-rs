use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ProxyScraperConfig {
    pub file: String,
    pub max: u32,
    pub timeout: u32
}

impl Default for ProxyScraperConfig {
    fn default() -> Self {
        ProxyScraperConfig {
            file: "proxies.txt".to_string(),
            max: 10,
            timeout: 5
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Account {
    pub username: String,
    pub password: String,
    pub default: bool
}

impl Default for Account {
    fn default() -> Self {
        Account {
            username: String::new(),
            password: String::new(),
            default: false
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AccountConfig {
    pub accounts: Vec<Account>
}

impl Default for AccountConfig {
    fn default() -> Self {
        AccountConfig {
            accounts: Vec::new()
        }
    }
}

impl Iterator for AccountConfig {
    type Item = Account;

    fn next(&mut self) -> Option<Self::Item> {
        self.accounts.iter().next().map(|a| a.clone())
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
#[allow(private_interfaces)] // since this won't be used outside of this module
pub struct BlacketConfig {
    pub accounts: AccountConfig,
    pub proxy_scraper: ProxyScraperConfig
}

impl Default for BlacketConfig {
    fn default() -> Self {
        BlacketConfig {
            accounts: AccountConfig::default(),
            proxy_scraper: ProxyScraperConfig::default()
        }
    }
}

pub fn parse_config(file_name: Option<&str>) -> BlacketConfig {
    let file = file_name.unwrap_or("config.toml");
    if fs::metadata(file).is_err() {
        panic!("config.toml not found");
    }
    toml::from_str(&fs::read_to_string(file).unwrap()).unwrap()
}

pub fn write_config(config: &BlacketConfig, file_name: Option<&str>) {
    let file = file_name.unwrap_or("config.toml");
    fs::write(file, toml::to_string(config).unwrap()).unwrap();
}