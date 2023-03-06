use anyhow::Result;
use std::net::{IpAddr, SocketAddr};

pub fn socket_addrs_from_args(ip_address: &str, port: u16) -> Result<SocketAddr> {
    let ip = ip_address.parse::<IpAddr>()?;
    Ok(SocketAddr::new(ip, port))
}
