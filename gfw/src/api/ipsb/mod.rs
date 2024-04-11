use std::{net::IpAddr, str::FromStr};

use reqwest::Url;
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://api.ip.sb";
const API_URL_IPV4: &str = "https://api-ipv4.ip.sb";
const API_URL_IPV6: &str = "https://api-ipv6.ip.sb";

pub async fn geoip(addr: Option<IpAddr>, version: Option<i8>) -> anyhow::Result<GeoIP> {
    let api_url = match version {
        Some(4) => API_URL_IPV4,
        Some(6) => API_URL_IPV6,
        None => API_URL,
        _ => return Err(anyhow::anyhow!("Invalid IP version")),
    };
    let url = if let Some(addr) = addr {
        Url::from_str(api_url)?.join(&format!("/geoip/{}", addr))?
    } else {
        Url::from_str(api_url)?.join("/geoip")?
    };
    let response = reqwest::get(url).await?.error_for_status()?;
    let response = response.json().await?;
    Ok(response)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoIP {
    pub asn: i32,
    pub country_code: String,
    pub country: String,
    pub ip: IpAddr,
    pub organization: String,
}
