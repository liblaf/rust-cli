use std::{
    net::{IpAddr, Ipv6Addr},
    str::FromStr,
};

use ipnet::Ipv6Net;
use once_cell::sync::Lazy;

pub async fn get_local_ips() -> Vec<IpAddr> {
    let mut ips = vec![];
    if let Ok(ip) = api::ipsb::ip(4).await {
        ips.push(ip);
    }
    if let Ok(ip) = api::ipsb::ip(6).await {
        ips.push(ip);
    }
    ips
}

pub fn is_global_ip(ip: &IpAddr) -> bool {
    if ip.is_loopback() {
        return false;
    }
    if ip.is_multicast() {
        return false;
    }
    if ip.is_unspecified() {
        return false;
    }
    if is_private_ip(ip) {
        return false;
    }
    true
}

pub fn is_private_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => ip.is_private(),
        IpAddr::V6(ip) => is_private_ipv6(ip),
    }
}

pub fn is_private_ipv6(ip: &Ipv6Addr) -> bool {
    static IPV6_PRIVATE_NETWORK: Lazy<Vec<Ipv6Net>> = Lazy::new(|| {
        vec![
            Ipv6Net::from_str("::1/128").unwrap(),
            Ipv6Net::from_str("::/128").unwrap(),
            Ipv6Net::from_str("::ffff:0:0/96").unwrap(),
            Ipv6Net::from_str("100::/64").unwrap(),
            Ipv6Net::from_str("2001::/23").unwrap(),
            Ipv6Net::from_str("2001:2::/48").unwrap(),
            Ipv6Net::from_str("2001:db8::/32").unwrap(),
            Ipv6Net::from_str("2001:10::/28").unwrap(),
            Ipv6Net::from_str("fc00::/7").unwrap(),
            Ipv6Net::from_str("fe80::/10").unwrap(),
        ]
    });
    IPV6_PRIVATE_NETWORK
        .iter()
        .any(|network| network.contains(ip))
}
