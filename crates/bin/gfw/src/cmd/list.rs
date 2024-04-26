use anyhow::Result;
use chrono::{DateTime, Local, Utc};
use clap::Args;
use colored::Colorize;
use indicatif::HumanBytes;
use reqwest::Url;
use tabled::builder::Builder;
use tabled::settings::object::Columns;
use tabled::settings::{Alignment, Color, Style};

use crate::config::Config;

#[derive(Args)]
pub struct Cmd {
    #[arg()]
    urls: Vec<Url>,
    #[arg(short, long, default_value_t = false)]
    url: bool,
    #[arg(long)]
    uuid: Option<String>,
}

impl Cmd {
    pub async fn run(&self) -> Result<()> {
        let config = Config::load()?;
        let info_list = if self.urls.is_empty() {
            let uuid = config.uuid(self.uuid.as_deref())?;
            api::liblaf::sub::info_by_uuid(&uuid).await?
        } else {
            api::liblaf::sub::info_by_urls(&self.urls).await?
        };
        let mut table = Builder::new();
        let table = if self.url {
            table.push_record(["Name", "URL"]);
            for info in info_list {
                table.push_record([info.name.as_str(), info.url.as_str()])
            }
            let mut table = table.build();
            table
                .with(Style::rounded())
                .modify(Columns::single(0), Color::FG_BLUE);
            table
        } else {
            table.push_record([
                "Name", "Upload", "Download", "Usage", "Remain", "Total", "Expire",
            ]);
            for info in info_list.as_slice() {
                if let (Some(download), Some(upload), Some(total), Some(expire)) =
                    (info.download, info.upload, info.total, info.expire)
                {
                    let usage = download + upload;
                    let ratio = usage as f64 / total as f64;
                    let color_bytes = if ratio < 0.6 {
                        colored::Color::Green
                    } else if ratio < 0.8 {
                        colored::Color::Yellow
                    } else {
                        colored::Color::Red
                    };
                    let remain = expire - Utc::now();
                    let color_date = if remain.num_days() > 14 {
                        colored::Color::Green
                    } else if remain.num_days() > 7 {
                        colored::Color::Yellow
                    } else {
                        colored::Color::Red
                    };
                    let format_bytes = |bytes: u64| -> String {
                        HumanBytes(bytes).to_string().color(color_bytes).to_string()
                    };
                    let format_date = |date: DateTime<Utc>| -> String {
                        date.with_timezone(&Local)
                            .format("%F")
                            .to_string()
                            .color(color_date)
                            .to_string()
                    };
                    table.push_record([
                        &info.name,
                        &format_bytes(upload),
                        &format_bytes(download),
                        &format_bytes(usage),
                        &format_bytes(total - usage),
                        &format_bytes(total),
                        &format_date(expire),
                    ])
                }
            }
            let mut table = table.build();
            table
                .with(Style::rounded())
                .with(Alignment::right())
                .modify(Columns::first(), Alignment::left())
                .modify(Columns::first(), tabled::settings::Color::FG_BLUE);
            table
        };
        println!("{}", table);
        Ok(())
    }
}
