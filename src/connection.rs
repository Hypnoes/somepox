use std::{net::SocketAddr, sync::Arc};

use bytes::{Bytes, BytesMut};
use tokio::net::UdpSocket;

use crate::error::GeneralError;

pub struct Connection {
    host: String,
    port: u16,
    connection: Option<Arc<UdpSocket>>,
}

impl Connection {
    pub fn new(host: &str, port: u16) -> Connection {
        Connection {
            host: host.to_string(),
            port: port,
            connection: None,
        }
    }

    pub async fn init(&mut self) -> Result<(), GeneralError> {
        let addr = format!("{}:{}", self.host, self.port);
        let raw_bind = UdpSocket::bind(addr).await?;
        self.connection = Some(Arc::new(raw_bind));
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), GeneralError> {
        self.connection = None;
        Ok(())
    }

    /// send a message
    ///
    /// FIXME: any message above 512 bytes will be dropped
    pub async fn send(
        &self,
        address: &str,
        data: Bytes,
    ) -> Result<(String, String, usize), GeneralError> {
        let addr = address
            .to_owned()
            .parse::<SocketAddr>()
            .map_err(|e| GeneralError::from(e.to_string()))?;

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
                Err("No Connection available.".to_owned().into())
            }
        }
    }

    /// recv message
    ///
    /// FIXME: any message above 512 bytes will be dropped
    pub async fn recv(&self) -> Result<(String, String, Bytes), GeneralError> {
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
                Err("No Connection available.".to_owned().into())
            }
        }
    }

    fn get_conn(&self) -> Option<Arc<UdpSocket>> {
        (self.connection.as_ref()).map(|socket_rc| socket_rc.clone())
    }
}
