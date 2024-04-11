use std::{collections::HashMap, net::IpAddr, str::FromStr};

use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://proxycheck.io";

pub async fn get(addr: IpAddr) -> anyhow::Result<Risk> {
    let client = Client::new();
    let url = Url::from_str(API_URL)?.join(&format!("/v2/{}", addr))?;
    let request = client
        .get(url)
        .query(&[("vpn", 3), ("asn", 1), ("risk", 2)]);
    let response = request.send().await?.error_for_status()?;
    let response: Response = response.json().await?;
    Ok(response.data.get(&addr).unwrap().to_owned())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Risk {
    pub risk: i8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Response {
    status: String,
    #[serde(flatten)]
    data: HashMap<IpAddr, Risk>,
}
