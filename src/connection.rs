use anyhow::{anyhow, Result};
use bytes::{Bytes, BytesMut};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::UdpSocket;

pub struct HostAndPort(pub String, pub u16);

impl TryFrom<String> for HostAndPort {
    type Error = anyhow::Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for HostAndPort {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(':').collect();

        if parts.len() != 2 {
            return Err(anyhow!("invalid host:port"));
        } else {
            let host = parts[0].to_string();
            if host.is_empty() {
                return Err(anyhow!("host is empty"));
            }

            let port = parts[1].parse::<u16>()?;
            if port < 5000 {
                return Err(anyhow!("port is beyond 5000"));
            }

            Ok(Self(host, port))
        }
    }
}

pub struct Connection {
    host: String,
    port: u16,
    connection: Option<Arc<UdpSocket>>,
}

impl Connection {
    pub fn new(endpoint: HostAndPort) -> Connection {
        Connection {
            host: endpoint.0,
            port: endpoint.1,
            connection: None,
        }
    }

    pub async fn init(&mut self) -> Result<()> {
        let addr = format!("{}:{}", self.host, self.port);
        let raw_bind = UdpSocket::bind(addr).await?;
        self.connection = Some(Arc::new(raw_bind));
        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        self.connection = None;
        Ok(())
    }

    /// send a message
    ///
    /// FIXME: any message above 512 bytes will be dropped
    pub async fn send(&self, address: &str, data: Bytes) -> Result<(String, String, usize)> {
        let addr = address.to_owned().parse::<SocketAddr>()?;

        let maybe_sock = self.get_conn();

        match maybe_sock {
            Some(sock) => {
                let buffer: &[u8] = &data;
                let record_size = sock.try_send_to(buffer, addr)?;

                let local_address = sock.local_addr()?.to_string();
                let remote_address = sock.peer_addr()?.to_string();

                Ok((local_address, remote_address, record_size))
            }
            None => {
                log::error!("no connection available.");
                Err(anyhow!("No Connection available."))
            }
        }
    }

    /// recv message
    ///
    /// FIXME: any message above 512 bytes will be dropped
    pub async fn recv(&self) -> Result<(String, String, Bytes)> {
        let mut msg = BytesMut::new();
        let maybe_sock = self.get_conn();

        match maybe_sock {
            Some(sock) => {
                let buffer: &mut [u8; 512] = &mut [0; 512];

                let (record_size, _) = sock.try_recv_from(buffer)?;

                msg.extend(&buffer[..record_size]);

                let local_address = sock.local_addr()?.to_string();
                let remote_address = sock.peer_addr()?.to_string();
                Ok((remote_address, local_address, msg.into()))
            }
            None => {
                log::error!("no connection available.");
                Err(anyhow!("No Connection available."))
            }
        }
    }

    fn get_conn(&self) -> Option<Arc<UdpSocket>> {
        (self.connection.as_ref()).map(|socket_rc| socket_rc.clone())
    }
}
