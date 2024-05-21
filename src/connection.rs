use anyhow::Result;
use bytes::Bytes;
use std::{
    net::UdpSocket,
    sync::{
        mpsc::{channel, Receiver},
        Arc, RwLock,
    },
    thread::{self, JoinHandle},
};

pub trait Connection {
    type Addr;
    fn send(&self, address: Self::Addr, data: Bytes) -> Result<(Self::Addr, Self::Addr, usize)>;
    fn recv(&self) -> Result<(String, String, Bytes)>;
}

pub struct Net {
    serv: Option<JoinHandle<()>>,
    serv_flag: Arc<RwLock<bool>>,
    local_addr: String,
    rx: Receiver<(String, Bytes)>,
    sock: Arc<UdpSocket>,
}

impl Net {
    pub fn new(endpoint: String) -> Result<Net> {
        let sc = Arc::new(UdpSocket::bind(endpoint.clone())?);

        let (tx, rx) = channel();
        let serv_flag = Arc::new(RwLock::new(true));
        let serv_flag_ref = serv_flag.clone();

        let sc_ref = sc.clone();
        let serv_handler = thread::Builder::new()
            .name("socket listener thread".to_string())
            .spawn(move || {
                let mut buffer = [0u8; 512];

                loop {
                    if let Ok(flag_ref) = serv_flag_ref.read() {
                        if !(flag_ref.clone()) {
                            break;
                        }
                        log::info!("Serv Shutdown.");
                    }
                    if let Ok((amt, src)) = sc_ref.clone().recv_from(&mut buffer) {
                        if let Ok(_) =
                            tx.send((src.to_string(), Bytes::copy_from_slice(&buffer[..amt])))
                        {
                        }
                    }
                }
            })?;

        Ok(Net {
            serv: Some(serv_handler),
            serv_flag: serv_flag.clone(),
            local_addr: endpoint,
            rx,
            sock: sc,
        })
    }

    fn serv_state(&self) -> bool {
        match self.serv_flag.clone().read() {
            Ok(v) => v.clone(),
            Err(_) => false,
        }
    }

    /// Shutdown The Serv
    ///
    /// FIXME: shutdown 有问题，join 会 block 在 recv_from 函数。使得循环永远不会执行至下一圈，所以无法到达判断 flag 处。
    pub fn shutdown(&mut self) -> Result<()> {
        let _ = self.serv_flag.write().map(|mut v| v.clone_from(&false));
        let _ = self.serv.take().map(|v| {
            let _ = v.join();
        });
        Ok(())
    }
}

impl Connection for Net {
    /// send a message
    ///
    /// FIXME: any message above 512 bytes will be dropped
    fn send(&self, address: Self::Addr, data: Bytes) -> Result<(Self::Addr, Self::Addr, usize)> {
        let buffer: &[u8] = &data;

        let sock = self.sock.clone();
        sock.connect(address.clone())?;
        let record_size = sock.send(buffer)?;
        let local_address = sock.local_addr()?.to_string();
        let remote_address = sock.peer_addr()?.to_string();
        Ok((local_address, remote_address, record_size))
    }

    /// recv message
    fn recv(&self) -> Result<(String, String, Bytes)> {
        let (remote_addr, data) = self.rx.recv()?;
        let local_addr = self.local_addr.clone();
        Ok((local_addr, remote_addr, data))
    }

    type Addr = String;
}

#[cfg(test)]
mod tests {

    use super::{Connection, Net};
    use anyhow::Result;

    #[test]
    fn test_1() {
        let test_message_1 = "test_message_1".to_string();
        let test_message_2 = "test_message_2".to_string();

        let test_addr_1 = "127.0.0.1:18001";
        let test_addr_2 = "127.0.0.1:18002";

        let test_conn_1 = Net::new(test_addr_1.to_string()).unwrap();
        let test_conn_2 = Net::new(test_addr_2.to_string()).unwrap();

        let mut send_result: Result<(String, String, usize)>;
        let mut recv_result: Result<()>;

        send_result = test_conn_1.send(test_addr_2.to_string(), test_message_1.clone().into());
        assert!(send_result.is_ok(), "Err = {}", send_result.unwrap_err());

        recv_result = test_conn_2.recv().and_then(|(_, _, v)| {
            let result = String::from_utf8(v.to_vec()).unwrap();
            assert!(test_message_1 == result);
            println!("conn 1 => conn 2 | OK.");
            Ok(())
        });
        assert!(recv_result.is_ok(), "Err = {}", recv_result.unwrap_err());

        send_result = test_conn_2.send(test_addr_1.to_string(), test_message_2.clone().into());
        assert!(send_result.is_ok(), "Err = {}", send_result.unwrap_err());

        recv_result = test_conn_1.recv().and_then(|(_, _, v)| {
            let result = String::from_utf8(v.to_vec()).unwrap();
            assert!(test_message_2 == result);
            println!("conn 2 => conn 1 | OK.");
            Ok(())
        });
        assert!(recv_result.is_ok(), "Err = {}", recv_result.unwrap_err());
    }
}
