mod initiator;
use std::{
    fmt::Display,
    net::{AddrParseError, SocketAddr},
    str::FromStr,
};

pub use initiator::*;
mod acceptor;
pub use acceptor::*;
mod reactor;
pub use reactor::*;
mod socket_settings;
pub use socket_settings::*;
mod stream_factory;
pub use stream_factory::*;

#[derive(Debug)]
pub(crate) enum ConnectionError {
    IOError(std::io::Error),
    AddrParseError(AddrParseError),
}
impl From<std::io::Error> for ConnectionError {
    fn from(e: std::io::Error) -> ConnectionError {
        ConnectionError::IOError(e)
    }
}
impl From<AddrParseError> for ConnectionError {
    fn from(e: AddrParseError) -> ConnectionError {
        ConnectionError::AddrParseError(e)
    }
}

impl Display for ConnectionError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionError::IOError(err) => {
                fmt.write_fmt(format_args!("Connection failed: {}", err))
            }
            ConnectionError::AddrParseError(err) => {
                fmt.write_fmt(format_args!("Failed to parse address: {}", err))
            }
        }
    }
}
