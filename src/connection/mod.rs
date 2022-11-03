mod initiator;
pub use initiator::*;
mod socket_settings;
pub use socket_settings::*;
mod stream_factory;
pub use stream_factory::*;

#[derive(Debug)]
pub enum ConnectionError {
    IOError(std::io::Error),
}
impl From<std::io::Error> for ConnectionError {
    fn from(e: std::io::Error) -> ConnectionError {
        ConnectionError::IOError(e)
    }
}
