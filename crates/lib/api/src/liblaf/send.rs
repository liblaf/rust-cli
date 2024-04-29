use reqwest::Client;
use serde::{Deserialize, Serialize};

pub async fn dns(chat_id: &str, body: &Body) -> anyhow::Result<()> {
    let url = format!("https://api.liblaf.me/bot/send/{}/dns", chat_id);
    let client = Client::new();
    let request = client.post(url).json(body);
    let response = request.send().await?;
    response.error_for_status_ref()?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DnsRecord {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Body {
    pub create: Vec<DnsRecord>,
    pub delete: Vec<DnsRecord>,
    pub keep: Vec<DnsRecord>,
}
