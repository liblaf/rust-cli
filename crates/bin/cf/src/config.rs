use clap::CommandFactory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            domain: Some(default_domain()),
            token: None,
            zone_id: None,
        }
    }
}

pub fn default_domain() -> String {
    format!("{}.ddns.liblaf.me", whoami::fallible::hostname().unwrap())
}

impl Config {
    pub fn load() -> Result<Config, confy::ConfyError> {
        let config: Config = confy::load(crate::cmd::Cmd::command().get_name(), "config")?;
        Ok(config)
    }

    pub fn domain<'a>(&self, domain: impl Into<Option<&'a str>>) -> anyhow::Result<String> {
        if let Some(domain) = domain.into() {
            Ok(domain.to_string())
        } else if let Some(domain) = &self.domain {
            Ok(domain.to_string())
        } else {
            anyhow::bail!("missing required argument: domain")
        }
    }

    pub fn token<'a>(&self, token: impl Into<Option<&'a str>>) -> anyhow::Result<String> {
        if let Some(token) = token.into() {
            Ok(token.to_string())
        } else if let Some(token) = &self.token {
            Ok(token.to_string())
        } else {
            anyhow::bail!("missing required argument: token")
        }
    }

    pub fn zone_id<'a>(&self, zone_id: impl Into<Option<&'a str>>) -> anyhow::Result<String> {
        if let Some(zone_id) = zone_id.into() {
            Ok(zone_id.to_string())
        } else if let Some(zone_id) = &self.zone_id {
            Ok(zone_id.to_string())
        } else {
            anyhow::bail!("missing required argument: zone_id")
        }
    }
}
