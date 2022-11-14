use crate::connection::ConnectionError;
use crate::connection::SocketSettings;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::time::Duration;

pub struct StreamFactory;
impl StreamFactory {
    pub fn create_client_stream(
        settings: &SocketSettings,
    ) -> Result<TcpStream, ConnectionError> {
        let endpoint: SocketAddr = settings.get_endpoint()?;
        let stream = TcpStream::connect(endpoint)?;
        StreamFactory::configure_stream(stream, settings).map_err(|e| e.into())
    }
    pub fn configure_stream(
        mut stream: TcpStream,
        settings: &SocketSettings,
    ) -> Result<TcpStream, std::io::Error> {
        // This is only okay because there is a timeout due to heartbeats,
        // otherwise it would be necessary to disconnect if read n == 0
        stream.set_read_timeout(Some(Duration::from_millis(1)));
        stream.set_write_timeout(Some(Duration::from_millis(1)));
        stream.set_nonblocking(true)?;
        Ok(stream)
    }
}
