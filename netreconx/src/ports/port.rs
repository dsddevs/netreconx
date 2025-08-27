use serde::Deserialize;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

#[derive(Debug, Deserialize, Clone)]
pub struct Port {
    pub port: u16,
    pub is_open: bool,
}

impl Port {
    pub(crate) fn connect_with_timeout(
        mut socket_addr: SocketAddr,
        port: u16,
        timeout: Duration,
    ) -> Port {
        socket_addr.set_port(port);
        let is_open = TcpStream::connect_timeout(&socket_addr, timeout).is_ok();
        Port { port, is_open }
    }
}



