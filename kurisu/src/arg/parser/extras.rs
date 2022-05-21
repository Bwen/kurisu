use crate::arg::Parser;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
impl Parser for IpAddr {
    fn parse(value: &'_ str) -> Self {
        if let Ok(ip) = value.parse::<Ipv6Addr>() {
            IpAddr::V6(ip)
        } else if let Ok(ip) = value.parse::<Ipv4Addr>() {
            IpAddr::V4(ip)
        } else {
            IpAddr::V4(Ipv4Addr::UNSPECIFIED)
        }
    }
}

use std::str::FromStr;
use url::Url;

impl Parser for Url {
    fn parse(value: &str) -> Self {
        if !value.is_empty() {
            if let Ok(url) = Url::from_str(value) {
                return url;
            } else if let Ok(url) = Url::from_str(format!("data:text/plain,?{}", value).as_str()) {
                return url;
            }
        }

        Url::from_str("data:text/plain,").unwrap()
    }
}
