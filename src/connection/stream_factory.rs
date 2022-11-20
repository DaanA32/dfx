use crate::connection::ConnectionError;
use crate::connection::SocketSettings;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::time::Duration;

pub(crate) struct StreamFactory;
impl StreamFactory {
    pub(crate) fn create_client_stream(
        settings: &SocketSettings,
    ) -> Result<TcpStream, ConnectionError> {
        let endpoint: SocketAddr = settings.get_endpoint()?;
        let stream = TcpStream::connect(endpoint)?;
        StreamFactory::configure_stream(stream, settings).map_err(|e| e.into())
    }
    pub(crate) fn configure_stream(
        stream: TcpStream,
        settings: &SocketSettings,
    ) -> Result<TcpStream, ConnectionError> {
        stream.set_read_timeout(match settings.receive_timeout() {
            0 => None,
            v => Some(Duration::from_millis(v))
        })?;
        stream.set_write_timeout(match settings.send_timeout() {
            0 => None,
            v => Some(Duration::from_millis(v))
        })?;
        stream.set_nodelay(settings.no_delay())?;
        // This is only okay because there is a timeout due to heartbeats,
        // otherwise it would be necessary to disconnect if read n == 0
        stream.set_nonblocking(true)?;
        Ok(stream)
    }
}
