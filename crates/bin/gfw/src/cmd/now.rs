use clap::{ArgAction, Args};
use colored::Color;
use colored::Colorize;
use console::Emoji;
use reqwest::Url;

use api::clash::Client;

#[derive(Args)]
pub struct Cmd {
    #[arg(short, long, default_value = "http://127.0.0.1:9090")]
    api: Url,
    #[arg(long, default_value_t = true, action(ArgAction::Set), default_missing_value("true"), num_args(0..=1), require_equals(true))]
    delay: bool,
    #[arg(long, default_value_t = true, action(ArgAction::Set), default_missing_value("true"), num_args(0..=1), require_equals(true))]
    emoji: bool,
}

const PROXY_NAME: &str = "PROXY";

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        let client = Client::new(self.api.as_ref());
        let proxies = client.proxies().await?;
        let mut proxy = proxies.get(PROXY_NAME).unwrap();
        let mut history = proxy.history.first();
        while proxy.type_ == "Selector" || proxy.type_ == "URLTest" {
            proxy = proxies.get(proxy.now.as_deref().unwrap()).unwrap();
            if let Some(h) = proxy.history.first() {
                history = Some(h);
            }
        }
        let delay = if self.delay {
            if let Some(history) = history {
                Some(history.delay)
            } else {
                match client
                    .proxies_delay(PROXY_NAME, "https://cp.cloudflare.com", 5000)
                    .await
                {
                    Ok(delay) => Some(delay),
                    Err(err) => {
                        tracing::error!("{}", err);
                        None
                    }
                }
            }
        } else {
            None
        };
        let mut output: String = String::new();
        if self.emoji {
            output.push_str(&Emoji("󰖟 ", " ").to_string());
        }
        output.push_str(&proxy.name);
        if self.delay {
            if let Some(delay) = delay {
                output.push_str(&format!(" {}ms", delay));
            } else {
                output.push_str(" N/A");
            }
        }
        let color = color_for_delay(delay);
        println!("{}", output.color(color));
        Ok(())
    }
}

fn color_for_delay(delay: Option<i64>) -> Color {
    match delay {
        // https://github.com/MetaCubeX/metacubexd/blob/0ca2f8329c2f99fde38a50d2e99b3c7c06448172/src/constants/index.ts#L88
        Some(0..=800) => Color::Green,
        Some(801..=1500) => Color::Yellow,
        _ => Color::Red,
    }
}
