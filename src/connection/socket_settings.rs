use std::{net::{SocketAddr, AddrParseError}, str::FromStr};

use super::ConnectionError;

#[derive(Debug, Clone)]
pub(crate) struct SocketSettings {
    host: String,
    port: u32,
    // TODO rest
}

impl SocketSettings {
    /// Creates a new [`SocketSettings`].
    pub(crate) fn new(host: String, port: u32) -> Self {
        Self {
            host,
            port
        }
    }

    pub(crate) fn get_endpoint(&self) -> Result<SocketAddr, ConnectionError> {
        let addr = format!("{}:{}", self.host, self.port);
        addr.parse().map_err(|v: AddrParseError| v.into())
    }
}
