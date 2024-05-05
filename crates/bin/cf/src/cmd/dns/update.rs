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
    #[arg(short, long)]
    chat_id: Option<String>,
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = Config::load()?;
        let name = config.domain(self.domain.as_deref())?;
        let token = config.token(self.token.as_deref())?;
        let zone_id = config.zone_id(self.zone_id.as_deref())?;
        let local_ips = crate::ip::local_ips().await;
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
            client.delete(&record.id, &zone_id).await?;
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
            if let Some(chat_id) = config.chat_id(self.chat_id.as_deref()) {
                api::liblaf::send::dns(
                    &chat_id,
                    &api::liblaf::send::Body {
                        create: to_create
                            .iter()
                            .map(|ip| api::liblaf::send::DnsRecord {
                                name: name.to_string(),
                                content: ip.to_string(),
                            })
                            .collect(),
                        delete: to_delete
                            .iter()
                            .map(|r| api::liblaf::send::DnsRecord {
                                name: name.to_string(),
                                content: r.content.to_string(),
                            })
                            .collect(),
                        keep: to_keep
                            .iter()
                            .map(|r| api::liblaf::send::DnsRecord {
                                name: name.to_string(),
                                content: r.content.to_string(),
                            })
                            .collect(),
                    },
                )
                .await?;
            }
        }
        Ok(())
    }
}
