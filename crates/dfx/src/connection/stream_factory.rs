use crate::connection::ConnectionError;
use crate::connection::SocketSettings;
use crate::session::SslOptions;
use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::time::Duration;

pub(crate) enum Stream {
    Tcp(TcpStream),
    Ssl(native_tls::TlsStream<TcpStream>),
}

impl Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Stream::Tcp(tcp) => tcp.read(buf),
            Stream::Ssl(ssl) => ssl.read(buf),
        }
    }
}

impl Write for Stream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Stream::Tcp(tcp) => tcp.write(buf),
            Stream::Ssl(ssl) => ssl.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Stream::Tcp(tcp) => tcp.flush(),
            Stream::Ssl(ssl) => ssl.flush(),
        }
    }
}

pub(crate) struct StreamFactory;
impl StreamFactory {
    pub(crate) fn create_client_stream(
        settings: SocketSettings,
    ) -> Result<Stream, ConnectionError> {
        let endpoint: SocketAddr = settings.get_endpoint()?;
        let stream = TcpStream::connect(endpoint)?;
        let stream = StreamFactory::configure_stream(stream, settings, false)?;
        Ok(stream)
    }
    pub(crate) fn configure_stream(
        mut stream: TcpStream,
        settings: SocketSettings,
        _acceptor: bool,
    ) -> Result<Stream, ConnectionError> {
        match settings.ssl_options() {
            Some(SslOptions::Acceptor { acceptor }) => {
                let mut stream = acceptor.accept(stream).unwrap();
                StreamFactory::configure_stream_mut(stream.get_mut(), &settings)?;
                Ok(Stream::Ssl(stream))
            }
            Some(SslOptions::Initiator { initiator, domain }) => {
                let mut stream = initiator.connect(domain, stream).unwrap();
                StreamFactory::configure_stream_mut(stream.get_mut(), &settings)?;
                Ok(Stream::Ssl(stream))
            }
            None => {
                StreamFactory::configure_stream_mut(&mut stream, &settings)?;
                Ok(Stream::Tcp(stream))
            }
        }
    }
    pub(crate) fn configure_stream_mut(
        stream: &mut TcpStream,
        settings: &SocketSettings,
    ) -> Result<(), ConnectionError> {
        stream.set_read_timeout(match settings.receive_timeout() {
            0 => None,
            v => Some(Duration::from_millis(v)),
        })?;
        stream.set_write_timeout(match settings.send_timeout() {
            0 => None,
            v => Some(Duration::from_millis(v)),
        })?;
        stream.set_nodelay(settings.no_delay())?;
        // This is only okay because there is a timeout due to heartbeats,
        // otherwise it would be necessary to disconnect if read n == 0
        stream.set_nonblocking(true)?;
        Ok(())
    }
}
