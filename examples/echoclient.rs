use anyhow::Result;
use std::{env, io, net::Ipv4Addr, str};
use toytcp::tcp::TCP;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let addr = args[1].parse::<Ipv4Addr>()?;
    let port = args[2].parse::<u16>()?;
    echo_client(addr, port)?;
    Ok(())
}

fn echo_client(remote_addr: Ipv4Addr, remote_port: u16) -> Result<()> {
    let tcp = TCP::new();
    let _ = tcp.connect(remote_addr, remote_port)?;
    Ok(())
}
