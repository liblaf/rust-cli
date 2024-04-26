use clap::CommandFactory;
use serde::{Deserialize, Serialize};

use crate::cmd::Cmd;

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    uuid: Option<String>,
}

impl Config {
    pub fn load() -> Result<Config, confy::ConfyError> {
        let cmd = Cmd::command();
        let config: Config = confy::load(cmd.get_name(), "config")?;
        Ok(config)
    }

    pub fn uuid<'a>(&self, uuid: impl Into<Option<&'a str>>) -> anyhow::Result<String> {
        if let Some(uuid) = uuid.into() {
            Ok(uuid.to_string())
        } else if let Some(uuid) = &self.uuid {
            Ok(uuid.to_string())
        } else {
            anyhow::bail!("missing required argument: uuid")
        }
    }
}
