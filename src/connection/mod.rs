mod initiator;
use std::{str::FromStr, net::{SocketAddr, AddrParseError}};

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
pub enum ConnectionError {
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
