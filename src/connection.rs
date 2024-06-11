#![allow(unused)]

use anyhow::Result;
use bytes::Bytes;
use core::hash;
use std::{
    net::{ToSocketAddrs, UdpSocket},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread::{self, JoinHandle},
};

/**
   ### Connection 抽象描述通信过程 ###

   Connection 主要通过两个方式实现通信 Send (发送) / Receive (接受)

   典型的实现包括三个:
   1. 线程：通过变量通信
   2. 进程：通过IPC通信
   3. 网络：通过Socket通信
*/
pub trait Connection {
    type Addr: Clone;
    fn address(&self) -> Self::Addr;
    fn send(&self, address: Self::Addr, data: Bytes) -> Result<(Self::Addr, Self::Addr, usize)>;
    fn recv(&self) -> Result<(Self::Addr, Self::Addr, Bytes)>;
}

pub struct Ipc {}

pub struct Channel {}

pub struct Net {
    sock: Arc<UdpSocket>,
    handler: Option<JoinHandle<()>>,
    channel: Receiver<(String, Bytes)>,
    addr: String,
}

impl Net {
    pub fn new(endpoint: String) -> Result<Net> {
        let sc = Arc::new(UdpSocket::bind(endpoint.clone())?);
        let (tx, rx) = channel();

        let sc_ref = sc.clone();
        let serv_handler = thread::Builder::new()
            .name("udp_socket".to_string())
            .spawn(move || {
                let mut buffer = [0u8; 512];

                loop {
                    if let Ok((amt, src)) = sc_ref.recv_from(&mut buffer) {
                        tx.send((src.to_string(), Bytes::copy_from_slice(&buffer[..amt])));
                    } else {
                        break;
                    };
                    buffer = [0u8; 512];
                }
            })?;

        Ok(Net {
            sock: sc,
            handler: Some(serv_handler),
            channel: rx,
            addr: endpoint,
        })
    }
}

impl Connection for Net {
    type Addr = String;

    fn address(&self) -> Self::Addr {
        self.addr.clone()
    }

    /// send a message
    ///
    /// FIXME: any message above 512 bytes will be dropped
    fn send(&self, address: Self::Addr, data: Bytes) -> Result<(Self::Addr, Self::Addr, usize)> {
        let buffer: &[u8] = &data;

        let sock = self.sock.clone();
        sock.connect(address.clone())?;
        let record_size = sock.send_to(buffer, address)?;
        let local_address = sock.local_addr()?.to_string();
        let remote_address = sock.peer_addr()?.to_string();
        Ok((local_address, remote_address, record_size))
    }

    /// recv message
    fn recv(&self) -> Result<(Self::Addr, Self::Addr, Bytes)> {
        let (remote_addr, data) = self.channel.recv()?;
        let local_addr = self.addr.clone();
        Ok((local_addr, remote_addr, data))
    }
}

impl Drop for Net {
    fn drop(&mut self) {
        let join_handler = self.handler.take();
        match join_handler {
            Some(handler) => handler.join(),
            None => Ok(()),
        };
    }
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
            assert_eq!(test_message_1, result);
            println!("conn 1 => conn 2 | OK.");
            Ok(())
        });
        assert!(recv_result.is_ok(), "Err = {}", recv_result.unwrap_err());

        send_result = test_conn_2.send(test_addr_1.to_string(), test_message_2.clone().into());
        assert!(send_result.is_ok(), "Err = {}", send_result.unwrap_err());

        recv_result = test_conn_1.recv().and_then(|(_, _, v)| {
            let result = String::from_utf8(v.to_vec()).unwrap();
            assert_eq!(test_message_2, result);
            println!("conn 2 => conn 1 | OK.");
            Ok(())
        });
        assert!(recv_result.is_ok(), "Err = {}", recv_result.unwrap_err());
    }
}
