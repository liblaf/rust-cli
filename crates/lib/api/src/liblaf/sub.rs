use chrono::{DateTime, Utc};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

pub async fn info_by_urls(urls: &[Url]) -> anyhow::Result<Vec<UserInfo>> {
    let client = Client::new();
    let request = client.get("https://api.liblaf.me/sub/info").query(
        &urls
            .iter()
            .map(|url| ("url", url.as_str()))
            .collect::<Vec<_>>(),
    );
    let response = request.send().await?.error_for_status()?;
    let response: Response = response.json().await?;
    Ok(response.info)
}

pub async fn info_by_uuid(uuid: &str) -> anyhow::Result<Vec<UserInfo>> {
    let client = Client::new();
    let request = client.get(format!("https://api.liblaf.me/sub/info/{}", uuid));
    let response = request.send().await?.error_for_status()?;
    let response: Response = response.json().await?;
    Ok(response.info)
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UserInfo {
    pub name: String,
    pub url: Url,
    pub web_page_url: Option<Url>,
    pub upload: Option<u64>,
    pub download: Option<u64>,
    pub total: Option<u64>,
    #[serde(default, with = "chrono::serde::ts_seconds_option")]
    pub expire: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct Response {
    info: Vec<UserInfo>,
}
