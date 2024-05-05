use std::net::IpAddr;

use clap::{ArgAction, Args};
use colored::{Color, Colorize};
use console::Emoji;
use tabled::builder::Builder;
use tabled::settings::object::Columns;
use tabled::settings::{Alignment, Concat, Style};
use tabled::Table;

use api::liblaf::ip::{Geo, IpInfo, Risk, Security};

#[derive(Args)]
pub struct Cmd {
    #[arg()]
    addr: Option<IpAddr>,
    #[arg(long, default_value_t = true, action(ArgAction::Set), default_missing_value("true"), num_args(0..=1), require_equals(true))]
    geo: bool,
    #[arg(long, default_value_t = true, action(ArgAction::Set), default_missing_value("true"), num_args(0..=1), require_equals(true))]
    risk: bool,
    #[arg(long, default_value_t = true, action(ArgAction::Set), default_missing_value("true"), num_args(0..=1), require_equals(true))]
    security: bool,
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        if let Some(addr) = self.addr {
            let info = get(Some(addr), None, self.geo, self.security, self.risk).await?;
            let table = create_table(&info)?;
            println!("{}", table);
        } else {
            let info4 = get(None, Some(4), self.geo, self.security, self.risk).await?;
            let mut table: Table = create_table(&info4)?;
            if let Ok(info6) = get(None, Some(6), self.geo, self.security, self.risk).await {
                let table6 = create_table(&info6)?;
                table.with(Concat::horizontal(table6));
            }
            println!("{}", table)
        }
        Ok(())
    }
}

async fn get(
    addr: Option<IpAddr>,
    version: Option<i8>,
    geo: bool,
    security: bool,
    risk: bool,
) -> anyhow::Result<IpInfo> {
    let addr = if let Some(addr) = addr {
        addr
    } else {
        api::ipsb::ip(version).await?
    };
    let info = api::liblaf::ip::info(addr, geo, risk, security).await?;
    Ok(info)
}

const DEFAULT_KEY_COLOR: Color = Color::BrightBlue;
const DEFAULT_VALUE_COLOR: Color = Color::BrightMagenta;

fn create_table(info: &IpInfo) -> anyhow::Result<Table> {
    let mut builder = Builder::new();
    builder.push_record([
        "IP".color(DEFAULT_KEY_COLOR).to_string(),
        info.ip.to_string().color(DEFAULT_VALUE_COLOR).to_string(),
    ]);
    if let Some(geo) = &info.geo {
        builder = create_table_geo(builder, geo)?;
    }
    if let Some(risk) = &info.risk {
        builder = create_table_risk(builder, risk)?;
    }
    if let Some(security) = &info.security {
        builder = create_table_security(builder, security)?;
    }
    let mut table = builder.build();
    table
        .with(Style::empty())
        .modify(Columns::first(), Alignment::right());
    Ok(table)
}

fn create_table_geo(mut builder: Builder, geo: &Geo) -> anyhow::Result<Builder> {
    builder.push_record([
        "ASN".color(DEFAULT_KEY_COLOR).to_string(),
        format!("AS{}", geo.asn)
            .color(DEFAULT_VALUE_COLOR)
            .to_string(),
    ]);

    builder.push_record([
        "Country".color(DEFAULT_KEY_COLOR).to_string(),
        format!(
            "{}{} ({})",
            Emoji(&format!("{} ", geo.country_flag), ""),
            &geo.country,
            &geo.country_code,
        )
        .color(DEFAULT_VALUE_COLOR)
        .to_string(),
    ]);

    builder.push_record([
        "Org".color(DEFAULT_KEY_COLOR).to_string(),
        geo.organization.color(DEFAULT_VALUE_COLOR).to_string(),
    ]);
    Ok(builder)
}

fn create_table_risk(mut builder: Builder, risk: &Risk) -> anyhow::Result<Builder> {
    let risk = risk.risk;
    let color = match risk {
        0..=33 => Color::BrightGreen,
        34..=66 => Color::BrightYellow,
        67..=100 => Color::BrightRed,
        _ => return Err(anyhow::anyhow!("Invalid risk value")),
    };
    builder.push_record([
        "Risk".color(color).to_string(),
        risk.to_string().color(color).to_string(),
    ]);
    Ok(builder)
}

fn get_color_bool(b: bool) -> Color {
    if b {
        Color::BrightRed
    } else {
        Color::BrightGreen
    }
}

fn create_table_security(mut builder: Builder, security: &Security) -> anyhow::Result<Builder> {
    let color = get_color_bool(security.abuser);
    builder.push_record([
        "Abuser".color(color).to_string(),
        security.abuser.to_string().color(color).to_string(),
    ]);

    let color = get_color_bool(security.data_center);
    builder.push_record([
        "Data Center".color(color).to_string(),
        security.data_center.to_string().color(color).to_string(),
    ]);

    let color = get_color_bool(security.proxy);
    builder.push_record([
        "Proxy".color(color).to_string(),
        security.proxy.to_string().color(color).to_string(),
    ]);

    let color = get_color_bool(security.tor);
    builder.push_record([
        "Tor".color(color).to_string(),
        security.tor.to_string().color(color).to_string(),
    ]);

    let color = get_color_bool(security.vpn);
    builder.push_record([
        "VPN".color(color).to_string(),
        security.vpn.to_string().color(color).to_string(),
    ]);

    Ok(builder)
}
