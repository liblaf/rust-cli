use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::clash::Client;

impl Client {
    pub async fn proxies(&self) -> anyhow::Result<HashMap<String, Proxy>> {
        let response = self.client.get(self.api.join("/proxies")?).send().await?;
        let response = response.error_for_status()?;
        let response: Response = response.json().await?;
        Ok(response.proxies)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Proxy {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    pub now: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Response {
    proxies: HashMap<String, Proxy>,
}
