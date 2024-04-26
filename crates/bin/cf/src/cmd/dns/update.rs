use std::{collections::HashSet, net::IpAddr, str::FromStr};

use api::cloudflare::{dns::records::RecordCreateParams, Cloudflare};
use clap::Args;

use crate::config::Config;

#[derive(Args)]
pub struct Cmd {
    #[arg(short, long)]
    domain: Option<String>,
    #[arg(from_global)]
    token: Option<String>,
    #[arg(from_global)]
    zone_id: Option<String>,
    #[arg(long)]
    telepush_token: Option<String>,
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = Config::load()?;
        let name = config.domain(self.domain.as_deref())?;
        let token = config.token(self.token.as_deref())?;
        let zone_id = config.zone_id(self.zone_id.as_deref())?;
        let local_ips = crate::ip::get_local_ips();
        let local_ips: HashSet<_> = local_ips
            .into_iter()
            .filter(crate::ip::is_global_ip)
            .collect();
        tracing::debug!("local IPs: {:?}", local_ips);
        let client = Cloudflare::new(&token);
        let client = client.dns().records();
        let remote_records = client.list(&zone_id).await?;
        let remote_records: Vec<_> = remote_records
            .into_iter()
            .filter(|r| r.name == name)
            .collect();
        tracing::debug!("remote records: {:?}", remote_records);
        let mut to_keep = vec![];
        let mut to_delete = vec![];
        for record in &remote_records {
            let ip = IpAddr::from_str(&record.content).unwrap();
            if local_ips.contains(&ip) {
                to_keep.push(record);
                tracing::debug!(addr = ?ip, name = name, "keep DNS record");
            } else {
                to_delete.push(record);
            }
        }
        let to_create: Vec<_> = local_ips
            .iter()
            .filter(|ip| !remote_records.iter().any(|r| r.content == ip.to_string()))
            .collect();
        for record in &to_delete {
            client.delete(&zone_id, &record.id).await?;
            tracing::info!(
                addr = record.content,
                name = record.name,
                "delete DNS record"
            );
        }
        for ip in &to_create {
            let type_ = match ip {
                IpAddr::V4(_) => "A",
                IpAddr::V6(_) => "AAAA",
            };
            client
                .create(
                    &zone_id,
                    &RecordCreateParams {
                        content: ip.to_string(),
                        name: name.to_string(),
                        proxied: Some(false),
                        type_: type_.to_string(),
                        ttl: Some(60),
                    },
                )
                .await?;
            tracing::info!(addr = ?ip, name = name, "create DNS record");
        }
        if !(to_delete.is_empty() && to_create.is_empty()) {
            if let Some(token) = self.telepush_token.as_deref() {
                let mut message = format!("*Domain*: {}", name);
                for record in to_keep {
                    message += &format!("\n- *KEEP* {}", record.content);
                }
                for record in to_delete {
                    message += &format!("\n- *DELETE* ~{}~", record.content);
                }
                for ip in to_create {
                    message += &format!("\n- *CREATE* *{}*", ip);
                }
                api::telepush::plain(token, message).await?;
            }
        }
        Ok(())
    }
}
