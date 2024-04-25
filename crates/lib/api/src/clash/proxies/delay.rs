use serde::{Deserialize, Serialize};

use crate::clash::Client;

impl Client {
    pub async fn proxies_delay(
        &self,
        proxy_name: &str,
        url: &str,
        timeout: i64,
    ) -> anyhow::Result<i64> {
        let request_url = self.api.join(&format!("/proxies/{}/delay", proxy_name))?;
        let response = self
            .client
            .get(request_url)
            .query(&[("url", url), ("timeout", timeout.to_string().as_str())])
            .send()
            .await?
            .error_for_status()?;
        let response: Response = response.json().await?;
        Ok(response.delay)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Response {
    pub delay: i64,
}
