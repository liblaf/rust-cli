use std::net::IpAddr;

use clap::{ArgAction, Args};
use colored::{Color, Colorize};
use console::Emoji;
use tabled::builder::Builder;
use tabled::settings::object::Columns;
use tabled::settings::{Alignment, Concat, Style};
use tabled::Table;

use crate::api::ipapiis::Security;
use crate::api::ipsb::GeoIP;
use crate::api::proxycheckio::Risk;

#[derive(Args)]
pub struct Cmd {
    #[arg()]
    addr: Option<IpAddr>,
    #[arg(long, default_value_t = true, action(ArgAction::Set), default_missing_value("true"), num_args(0..=1), require_equals(true))]
    risk: bool,
    #[arg(long, default_value_t = true, action(ArgAction::Set), default_missing_value("true"), num_args(0..=1), require_equals(true))]
    security: bool,
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        if let Some(addr) = self.addr {
            let info = get(Some(addr), None, self.security, self.risk).await?;
            let table = create_table(&info)?;
            println!("{}", table);
        } else {
            let info4 = get(None, Some(4), self.security, self.risk).await?;
            let mut table: Table = create_table(&info4)?;
            if let Ok(info6) = get(None, Some(6), self.security, self.risk).await {
                let table6 = create_table(&info6)?;
                table.with(Concat::horizontal(table6));
            }
            println!("{}", table)
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct IpInfo {
    geoip: GeoIP,
    risk: Option<Risk>,
    security: Option<Security>,
}

async fn get(
    addr: Option<IpAddr>,
    version: Option<i8>,
    security: bool,
    risk: bool,
) -> anyhow::Result<IpInfo> {
    let geoip = get_geoip(addr, version).await?;
    let addr = geoip.ip;
    Ok(IpInfo {
        geoip,
        risk: if risk {
            get_risk(addr).await.ok()
        } else {
            None
        },
        security: if security {
            get_security(addr).await.ok()
        } else {
            None
        },
    })
}

fn create_table(info: &IpInfo) -> anyhow::Result<Table> {
    let builder = Builder::new();
    let mut builder = create_table_geoip(builder, &info.geoip)?;
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

async fn get_geoip(addr: Option<IpAddr>, version: Option<i8>) -> anyhow::Result<GeoIP> {
    let geoip = crate::api::ipsb::geoip(addr, version).await?;
    Ok(geoip)
}

async fn get_risk(addr: IpAddr) -> anyhow::Result<Risk> {
    let risk = crate::api::proxycheckio::get(addr).await?;
    Ok(risk)
}

async fn get_security(addr: IpAddr) -> anyhow::Result<Security> {
    let security = crate::api::ipapiis::get(Some(addr)).await?;
    Ok(security)
}

const DEFAULT_KEY_COLOR: Color = Color::BrightBlue;
const DEFAULT_VALUE_COLOR: Color = Color::BrightMagenta;

fn create_table_geoip(mut builder: Builder, geoip: &GeoIP) -> anyhow::Result<Builder> {
    builder.push_record([
        "IP".color(DEFAULT_KEY_COLOR).to_string(),
        geoip.ip.to_string().color(DEFAULT_VALUE_COLOR).to_string(),
    ]);

    builder.push_record([
        "ASN".color(DEFAULT_KEY_COLOR).to_string(),
        format!("AS{}", geoip.asn)
            .color(DEFAULT_VALUE_COLOR)
            .to_string(),
    ]);

    let emoji = geoip
        .country_code
        .chars()
        .map(|c| char::from_u32(c as u32 + 127397).unwrap())
        .collect::<String>();
    builder.push_record([
        "Country".color(DEFAULT_KEY_COLOR).to_string(),
        format!(
            "{}{} ({})",
            Emoji(&format!("{} ", emoji), ""),
            &geoip.country,
            &geoip.country_code,
        )
        .color(DEFAULT_VALUE_COLOR)
        .to_string(),
    ]);

    builder.push_record([
        "Org".color(DEFAULT_KEY_COLOR).to_string(),
        geoip.organization.color(DEFAULT_VALUE_COLOR).to_string(),
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

fn create_table_security(mut builder: Builder, security: &Security) -> anyhow::Result<Builder> {
    let get_color = |b| {
        if b {
            Color::BrightRed
        } else {
            Color::BrightGreen
        }
    };

    let color = get_color(security.is_abuser);
    builder.push_record([
        "Abuser".color(color).to_string(),
        security.is_abuser.to_string().color(color).to_string(),
    ]);

    let color = get_color(security.is_datacenter);
    builder.push_record([
        "Data Center".color(color).to_string(),
        security.is_datacenter.to_string().color(color).to_string(),
    ]);

    let color = get_color(security.is_proxy);
    builder.push_record([
        "Proxy".color(color).to_string(),
        security.is_proxy.to_string().color(color).to_string(),
    ]);

    let color = get_color(security.is_tor);
    builder.push_record([
        "Tor".color(color).to_string(),
        security.is_tor.to_string().color(color).to_string(),
    ]);

    let color = get_color(security.is_vpn);
    builder.push_record([
        "VPN".color(color).to_string(),
        security.is_vpn.to_string().color(color).to_string(),
    ]);

    Ok(builder)
}
