use api::cloudflare::Cloudflare;
use clap::Args;

#[derive(Args)]
pub struct Cmd {
    #[arg(short, long)]
    name: Option<String>,
    #[arg(from_global)]
    api_url: String,
    #[arg(from_global)]
    token: Option<String>,
    #[arg(from_global)]
    zone_id: String,
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        let name = crate::args::get_domain(self.name.as_deref()).await?;
        let token = crate::args::get_token(self.token.as_deref()).await?;
        let client = Cloudflare::new(Some(&self.api_url), &token);
        let client = client.dns().records();
        let records = client.list(&self.zone_id).await?;
        let records: Vec<_> = records.iter().filter(|r| r.name == name).collect();
        let table = crate::fmt::dns_record_table(&records);
        println!("{}", table);
        Ok(())
    }
}
