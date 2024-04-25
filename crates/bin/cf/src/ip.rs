use std::{
    net::{IpAddr, Ipv6Addr},
    str::FromStr,
};

use once_cell::sync::Lazy;
use pnet::ipnetwork::Ipv6Network;

pub fn get_local_ips() -> Vec<IpAddr> {
    pnet::datalink::interfaces()
        .iter()
        .flat_map(|iface| iface.ips.iter().map(|network| network.ip()))
        .collect()
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
    static IPV4_PRIVATE_NETWORK: Lazy<Vec<Ipv6Network>> = Lazy::new(|| {
        vec![
            Ipv6Network::from_str("::1/128").unwrap(),
            Ipv6Network::from_str("::/128").unwrap(),
            Ipv6Network::from_str("::ffff:0:0/96").unwrap(),
            Ipv6Network::from_str("100::/64").unwrap(),
            Ipv6Network::from_str("2001::/23").unwrap(),
            Ipv6Network::from_str("2001:2::/48").unwrap(),
            Ipv6Network::from_str("2001:db8::/32").unwrap(),
            Ipv6Network::from_str("2001:10::/28").unwrap(),
            Ipv6Network::from_str("fc00::/7").unwrap(),
            Ipv6Network::from_str("fe80::/10").unwrap(),
        ]
    });
    IPV4_PRIVATE_NETWORK
        .iter()
        .any(|network| network.contains(ip.to_owned()))
}
