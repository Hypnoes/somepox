use std::{error::Error, fmt::Display, sync::Arc};

use log::{error, info};
use tokio::net::UdpSocket;

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
        let addr = String::from(&self.host) + ":" + &self.port.to_string();

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

    pub async fn close(&mut self) -> Result<(), Box<dyn Error>> {
        self.connection = None;
        Ok(())
    }

    /// send a message
    ///
    /// FIXME: any message above 512 bytes will be dropped
    pub async fn send(&self, address: &str, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
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
            Err(Box::new(ConnectionError {
                msg: "cccc".to_string(),
            }))
        }
    }

    /// recv message
    ///
    /// FIXME: any message above 512 bytes will be dropped
    pub async fn recv(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut msg: Vec<u8> = Vec::new();
        let conn_ref = self.get_conn();

        match conn_ref {
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

#[derive(Debug)]
struct ConnectionError {
    msg: String,
}

impl Error for ConnectionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionError(msg: {})", self.msg)
    }
}
