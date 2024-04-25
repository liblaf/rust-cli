pub mod dns;

use std::fmt::Debug;

use reqwest::{header::HeaderMap, ClientBuilder, RequestBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub const API: &str = "https://api.cloudflare.com/client/v4";

#[derive(Clone)]
pub struct Cloudflare {
    api: String,
    client: reqwest::Client,
    token: String,
}

impl Cloudflare {
    pub fn new(api: Option<&str>, token: &str) -> Self {
        Self {
            api: api.unwrap_or(API).to_string(),
            client: ClientBuilder::new()
                .default_headers({
                    let mut headers = HeaderMap::new();
                    headers.insert(
                        reqwest::header::AUTHORIZATION,
                        format!("Bearer {}", token).parse().unwrap(),
                    );
                    headers
                })
                .build()
                .unwrap(),
            token: token.to_string(),
        }
    }

    pub async fn send<T>(&self, request: RequestBuilder) -> anyhow::Result<T>
    where
        T: Debug + DeserializeOwned,
    {
        let request = request.bearer_auth(&self.token);
        let response = request.send().await?;
        let response = response.error_for_status()?;
        let response: Response<T> = response.json().await?;
        if !response.success {
            let mut message = String::new();
            for error in &response.errors {
                message += &format!("{}: {}\n", error.code, error.message);
            }
            for msg in &response.messages {
                message += &format!("{}: {}\n", msg.code, msg.message);
            }
            let message = message.trim().to_string();
            return Err(anyhow::anyhow!(message));
        }
        let result = response
            .result
            .ok_or(anyhow::anyhow!("No result in response"))?;
        Ok(result)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response<T> {
    errors: Vec<Message>,
    messages: Vec<Message>,
    result: Option<T>,
    success: bool,
    result_info: Option<ResultInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    code: i64,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResultInfo {
    count: i64,
    page: i64,
    per_page: i64,
    total_count: i64,
}
