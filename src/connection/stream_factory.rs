use crate::connection::ConnectionError;
use crate::connection::SocketSettings;
use std::net::SocketAddr;
use std::net::TcpStream;

pub struct StreamFactory;
impl StreamFactory {
    pub fn create_client_stream(
        endpoint: &SocketAddr,
        settings: &SocketSettings,
    ) -> Result<TcpStream, ConnectionError> {
        Ok(TcpStream::connect(endpoint)?)
    }
}
