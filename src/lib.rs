/**
Crate Doc Here.
*/
mod connection;
mod issue;
mod logbackend;
mod mail;
mod roles;

pub const HALF_OF_VOTERS: u8 = 1;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

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

        let president = President::new("localhost:5001".to_string(), addr_book.clone()).unwrap();
        let proposer = Proposer::new("localhost:5002".to_string(), addr_book.clone()).unwrap();
        let secretary =
            Secretary::new("localhost:5003".to_string(), addr_book.clone(), log_backend).unwrap();
        let senator1 = Senator::new("localhost:5004".to_string(), addr_book.clone()).unwrap();
        let senator2 = Senator::new("localhost:5005".to_string(), addr_book.clone()).unwrap();
        let senator3 = Senator::new("localhost:5006".to_string(), addr_book.clone()).unwrap();

        let proposer_emmit_issue = proposer.emmit("test-msg:A");
        assert!(proposer_emmit_issue.is_ok());

        println!("{:?}", secretary.get_log(0));
    }
}
