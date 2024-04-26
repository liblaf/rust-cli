use api::cloudflare::Cloudflare;
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
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = Config::load()?;
        let domain = config.domain(self.domain.as_deref())?;
        let token = config.token(self.token.as_deref())?;
        let zone_id = config.zone_id(self.zone_id.as_deref())?;
        dbg!(&token);

        let client = Cloudflare::new(&token);
        let client = client.dns().records();
        let records = client.list(&zone_id).await?;
        let records: Vec<_> = records.iter().filter(|r| r.name == domain).collect();
        let table = crate::fmt::dns_record_table(&records);
        println!("{}", table);
        Ok(())
    }
}
