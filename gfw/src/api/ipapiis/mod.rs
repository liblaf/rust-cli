use std::net::IpAddr;

use reqwest::Client;
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://api.ipapi.is";

pub async fn get(addr: Option<IpAddr>) -> anyhow::Result<Security> {
    let client = Client::new();
    let mut request = client.get(API_URL);
    if let Some(addr) = addr {
        request = request.query(&[("q", addr.to_string())]);
    }
    let response = request.send().await?.error_for_status()?;
    let response: Security = response.json().await?;
    Ok(response)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Security {
    pub ip: IpAddr,
    pub is_abuser: bool,
    pub is_datacenter: bool,
    pub is_proxy: bool,
    pub is_tor: bool,
    pub is_vpn: bool,
}
