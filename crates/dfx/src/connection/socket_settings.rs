use std::net::SocketAddr;

use crate::session::{SocketOptions, SslOptions};

use super::ConnectionError;

#[derive(Debug, Clone)]
pub(crate) struct SocketSettings {
    addr: SocketAddr,
    no_delay: bool,
    // send_buffer_size: usize,
    // receive_buffer_size: usize,
    send_timeout: u64,
    receive_timeout: u64,
    ssl_options: Option<SslOptions>,
}

impl SocketSettings {
    /// Creates a new [`SocketSettings`].
    pub(crate) fn new(
        socket_addr: SocketAddr,
        socket_options: SocketOptions,
        ssl_options: Option<SslOptions>,
    ) -> Self {
        Self {
            addr: socket_addr,
            no_delay: socket_options.no_delay(),
            // send_buffer_size: socket_options.send_buffer_size(),
            // receive_buffer_size: socket_options.receive_buffer_size(),
            send_timeout: socket_options.send_timeout(),
            receive_timeout: socket_options.receive_timeout(),
            ssl_options,
        }
    }

    pub(crate) fn get_endpoint(&self) -> Result<SocketAddr, ConnectionError> {
        Ok(self.addr)
    }

    pub(crate) fn no_delay(&self) -> bool {
        self.no_delay
    }

    // pub(crate) fn send_buffer_size(&self) -> usize {
    //     self.send_buffer_size
    // }

    // pub(crate) fn receive_buffer_size(&self) -> usize {
    //     self.receive_buffer_size
    // }

    pub(crate) fn send_timeout(&self) -> u64 {
        self.send_timeout
    }

    pub(crate) fn receive_timeout(&self) -> u64 {
        self.receive_timeout
    }

    pub(crate) fn ssl_options(&self) -> &Option<SslOptions> {
        &self.ssl_options
    }
}
