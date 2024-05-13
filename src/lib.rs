pub mod connection;
pub mod issue;
pub mod logbackend;
pub mod mail;
pub mod roles;

pub const HALF_OF_VOTERS: u8 = 1;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        logbackend::file_logbackend,
        roles::{
            new_president, new_proposer, new_secretary, new_senator, PRESIDENT_ROLE_NAME,
            PROPOSER_ROLE_NAME, SECRETARY_ROLE_NAME, SENATOR_ROLE_NAME,
        },
    };

    #[test]
    fn test_1() {
        let mut addr_book = HashMap::new();

        addr_book.insert(
            PRESIDENT_ROLE_NAME.to_string(),
            vec!["localhost:5001".to_string()],
        );
        addr_book.insert(
            PROPOSER_ROLE_NAME.to_string(),
            vec!["localhost:5002".to_string()],
        );
        addr_book.insert(
            SECRETARY_ROLE_NAME.to_string(),
            vec!["localhost:5003".to_string()],
        );
        addr_book.insert(
            SENATOR_ROLE_NAME.to_string(),
            vec![
                "localhost:5005".to_string(),
                "localhost:5006".to_string(),
                "localhost:5007".to_string(),
            ],
        );

        let log_backend = file_logbackend();

        let president = new_president(addr_book.clone(), "localhost:5001").unwrap();
        let proposer = new_proposer(addr_book.clone(), "localhost:5002").unwrap();
        let secretary = new_secretary(addr_book.clone(), "localhost:5003", log_backend).unwrap();
        let senator1 = new_senator(addr_book.clone(), "localhost:5005").unwrap();
        let senator2 = new_senator(addr_book.clone(), "localhost:5006").unwrap();
        let senator3 = new_senator(addr_book.clone(), "localhost:5007").unwrap();

        assert!(true, "test");
    }
}
