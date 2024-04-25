use std::{net::IpAddr, str::FromStr};

const API_URL: &str = "https://api.ip.sb";
const API_URL_IPV4: &str = "https://api-ipv4.ip.sb";
const API_URL_IPV6: &str = "https://api-ipv6.ip.sb";

pub async fn ip(version: Option<i8>) -> anyhow::Result<IpAddr> {
    let api_url = match version {
        Some(4) => API_URL_IPV4,
        Some(6) => API_URL_IPV6,
        None => API_URL,
        _ => return Err(anyhow::anyhow!("Invalid IP version")),
    };
    let url = format!("{api_url}/ip");
    let response = reqwest::get(url).await?.error_for_status()?;
    let response = response.text().await?;
    Ok(IpAddr::from_str(response.trim())?)
}
