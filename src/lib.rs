mod connection;
mod issue;
mod logbackend;
mod mail;
mod roles;

pub const HALF_OF_VOTERS: u8 = 1;

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, net::UdpSocket};

    use crate::{
        logbackend::HeapLogBackend,
        roles::{President, Proposer, Secretary, Senator},
    };

    #[test]
    fn test_1() {
        let mut addr_book = HashMap::new();

        addr_book.insert("president".to_string(), vec!["localhost:5001".to_string()]);
        addr_book.insert("proposer".to_string(), vec!["localhost:5002".to_string()]);
        addr_book.insert("secretary".to_string(), vec!["localhost:5003".to_string()]);
        addr_book.insert(
            "senator".to_string(),
            vec![
                "localhost:5005".to_string(),
                "localhost:5006".to_string(),
                "localhost:5007".to_string(),
            ],
        );

        let log_backend = HeapLogBackend::new();

        let president = President::new("localhost:5001".to_string()).unwrap();
        let proposer = Proposer::new("localhost:5002".to_string()).unwrap();
        let secretary = Secretary::new("localhost:5003".to_string(), log_backend).unwrap();
        let senator1 = Senator::new("localhost:5004".to_string()).unwrap();
        let senator2 = Senator::new("localhost:5005".to_string()).unwrap();
        let senator3 = Senator::new("localhost:5006".to_string()).unwrap();

        let mut result = proposer.emmit("test-msg:A");

        assert!(result.is_ok());
    }

    #[test]
    fn test_2() -> std::io::Result<()> {
        {
            let socket = UdpSocket::bind("127.0.0.1:34254")?;

            // Receives a single datagram message on the socket. If `buf` is too small to hold
            // the message, it will be cut off.
            let mut buf = [0; 10];
            let (amt, src) = socket.recv_from(&mut buf)?;

            // Redeclare `buf` as slice of the received data and send reverse data back to origin.
            let buf = &mut buf[..amt];
            buf.reverse();
            socket.send_to(buf, &src)?;
        } // the socket is closed here
        Ok(())
    }
}
