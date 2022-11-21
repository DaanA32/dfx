mod initiator;
use std::{
    fmt::Display,
    net::AddrParseError,
};

pub use initiator::*;
mod acceptor;
pub use acceptor::*;
mod reactor;
use openssl::error::ErrorStack;
pub(crate) use reactor::*;
mod socket_settings;
pub(crate) use socket_settings::*;
mod stream_factory;
pub(crate) use stream_factory::*;

#[derive(Debug)]
pub(crate) enum ConnectionError {
    IOError(std::io::Error),
    AddrParseError(AddrParseError),
    SslErrorStack(ErrorStack),
    SslError(openssl::ssl::Error),
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
impl From<ErrorStack> for ConnectionError {
    fn from(e: ErrorStack) -> ConnectionError {
        ConnectionError::SslErrorStack(e)
    }
}
impl From<openssl::ssl::Error> for ConnectionError {
    fn from(e: openssl::ssl::Error) -> ConnectionError {
        ConnectionError::SslError(e)
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
            ConnectionError::SslErrorStack(err) => {
                fmt.write_fmt(format_args!("SSL failed: {}", err))
            },
            ConnectionError::SslError(err) => {
                fmt.write_fmt(format_args!("SSL failed: {}", err))
            },
        }
    }
}
