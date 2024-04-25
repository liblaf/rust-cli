mod proxies;

use reqwest::{IntoUrl, Url};

pub struct Client {
    api: Url,
    client: reqwest::Client,
}

impl Client {
    pub fn new<U>(api: U) -> Self
    where
        U: IntoUrl,
    {
        Self {
            api: api.into_url().unwrap(),
            client: reqwest::Client::new(),
        }
    }
}
