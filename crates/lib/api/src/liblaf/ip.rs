use std::net::IpAddr;

use reqwest::Client;
use serde::{Deserialize, Serialize};

pub async fn info(ip: IpAddr, geo: bool, risk: bool, security: bool) -> anyhow::Result<IpInfo> {
    let client = Client::new();
    let request = client
        .get(format!("https://api.liblaf.me/ip/info/{}", ip))
        .query(&[("geo", geo), ("risk", risk), ("security", security)]);
    let response = request.send().await?.error_for_status()?;
    let response: IpInfo = response.json().await?;
    Ok(response)
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IpInfo {
    pub ip: IpAddr,
    pub geo: Option<Geo>,
    pub risk: Option<Risk>,
    pub security: Option<Security>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Geo {
    pub asn: u32,
    pub country: String,
    pub country_code: String,
    pub country_flag: String,
    pub latitude: f64,
    pub longitude: f64,
    pub organization: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Risk {
    pub risk: u8,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Security {
    pub abuser: bool,
    pub crawler: bool,
    pub data_center: bool,
    pub proxy: bool,
    pub tor: bool,
    pub vpn: bool,
}
