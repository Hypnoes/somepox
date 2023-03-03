use std::sync::Arc;

use log::{error, info};
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

    pub async fn init(&mut self) {
        let addr = format!("{}:{}", self.host, self.port);

        self.connection = {
            let socket = UdpSocket::bind(addr).await;
            match socket {
                Ok(r) => Some(Arc::new(r)),
                Err(e) => {
                    error!("{}", e);
                    None
                }
            }
        }
    }

    pub async fn close(&mut self) -> Result<(), GeneralError> {
        self.connection = None;
        Ok(())
    }

    /// send a message
    ///
    /// FIXME: any message above 512 bytes will be dropped
    pub async fn send(&self, address: &str, data: Vec<u8>) -> Result<(), GeneralError> {
        if data.len() <= 512 {
            let buffer: &[u8] = &data;

            let conn_ref = self.get_conn();

            match conn_ref {
                Some(conn) => {
                    let result = conn.send_to(buffer, address).await?;
                    info!("send {} bytes message.", result);
                }
                None => {
                    error!("no connection available.");
                }
            }

            Ok(())
        } else {
            Err("data is too large".into())
        }
    }

    /// recv message
    ///
    /// FIXME: any message above 512 bytes will be dropped
    pub async fn recv(&self) -> Result<Vec<u8>, GeneralError> {
        let mut msg: Vec<u8> = Vec::new();

        match self.get_conn() {
            Some(conn) => {
                let buffer: &mut [u8; 512] = &mut [0; 512];
                let record_size = conn.recv(buffer).await?;
                msg.extend(&buffer[..record_size]);
                info!("recv {} bytes message.", record_size);
            }
            None => error!("no connection available."),
        }

        Ok(msg)
    }

    fn get_conn(&self) -> Option<Arc<UdpSocket>> {
        self.connection.as_ref().map(|cc| cc.clone())
    }
}
