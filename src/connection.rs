use anyhow::Result;
use bytes::{Bytes, BytesMut};
use std::{
    net::{ToSocketAddrs, UdpSocket},
    sync::Arc,
};

pub trait Connection {
    type Addr;
    fn send(&self, address: Self::Addr, data: Bytes) -> Result<(Self::Addr, Self::Addr, usize)>;
    fn recv(&self) -> Result<(Self::Addr, Self::Addr, Bytes)>;
}

pub struct Net(Arc<UdpSocket>);

impl Net {
    pub fn new(endpoint: impl ToSocketAddrs) -> Result<Net> {
        Ok(Net(Arc::new(UdpSocket::bind(endpoint)?)))
    }

    fn get_conn(&self) -> *const UdpSocket {
        Arc::downgrade(&self.0).into_raw()
    }
}

impl Connection for Net {
    /// send a message
    ///
    /// FIXME: any message above 512 bytes will be dropped
    fn send(&self, address: Self::Addr, data: Bytes) -> Result<(Self::Addr, Self::Addr, usize)> {
        let buffer: &[u8] = &data;

        unsafe {
            let sock = &*(self.get_conn());
            let record_size = sock.send_to(buffer, address)?;
            let local_address = sock.local_addr()?.to_string();
            let remote_address = sock.peer_addr()?.to_string();
            Ok((local_address, remote_address, record_size))
        }
    }

    /// recv message
    ///
    /// FIXME: any message above 512 bytes will be dropped
    fn recv(&self) -> Result<(Self::Addr, Self::Addr, Bytes)> {
        let mut msg = BytesMut::new();
        let buffer: &mut [u8; 512] = &mut [0; 512];

        unsafe {
            let sock = &*(self.get_conn());
            let (record_size, addr) = sock.recv_from(buffer)?;
            msg.extend(&buffer[..record_size]);

            let local_address = sock.local_addr()?.to_string();
            let remote_address = sock.peer_addr()?.to_string();
            Ok((remote_address, local_address, msg.into()))
        }
    }

    type Addr = String;
}

#[cfg(test)]
mod tests {

    use std::net::UdpSocket;

    use super::{Connection, Net};
    use anyhow::Result;

    #[test]
    fn test_1() {
        let test_message_1 = "test_message_1".to_string();
        let test_message_2 = "test_message_2".to_string();

        let test_addr_1 = "127.0.0.1:18001";
        let test_addr_2 = "127.0.0.1:18002";

        let test_conn_1 = Net::new(test_addr_1).unwrap();
        let test_conn_2 = Net::new(test_addr_2).unwrap();

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

    #[test]
    fn test_2() {
        let socket = UdpSocket::bind("127.0.0.1:34254").unwrap();

        let mut buf = [0; 10];
        let (amt, src) = socket.recv_from(&mut buf).unwrap();

        let buf = &mut buf[..amt];
        buf.reverse();
        socket.send_to(buf, &src).unwrap();
    }
}
