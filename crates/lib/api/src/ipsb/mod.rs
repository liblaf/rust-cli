use std::{net::IpAddr, str::FromStr};

const API_URL: &str = "https://api.ip.sb";
const API_URL_IPV4: &str = "https://api-ipv4.ip.sb";
const API_URL_IPV6: &str = "https://api-ipv6.ip.sb";

pub async fn ip<V>(version: V) -> anyhow::Result<IpAddr>
where
    V: Into<Option<i8>>,
{
    let api_url = get_api_url(version);
    let url = format!("{api_url}/ip");
    let response = reqwest::get(url).await?;
    let response = response.error_for_status()?;
    let response = response.text().await?;
    Ok(IpAddr::from_str(response.trim())?)
}

fn get_api_url<V>(version: V) -> &'static str
where
    V: Into<Option<i8>>,
{
    match version.into() {
        Some(4) => API_URL_IPV4,
        Some(6) => API_URL_IPV6,
        None => API_URL,
        _ => unreachable!(),
    }
}
