use clap::Args;
use reqwest::Url;

use crate::api::clash::Client;

#[derive(Args)]
pub struct Cmd {
    #[arg(short, long, default_value = "http://127.0.0.1:9090")]
    api: Url,
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        let client = Client::new(self.api.as_ref());
        let proxies = client.proxies().await?;
        let mut proxy = proxies.get("PROXY").unwrap();
        while proxy.type_ == "Selector" || proxy.type_ == "URLTest" {
            proxy = proxies.get(proxy.now.as_deref().unwrap()).unwrap();
        }
        println!("{}", proxy.name);
        Ok(())
    }
}
