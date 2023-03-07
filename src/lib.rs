pub mod connection;
pub mod logbackend;
pub mod mail;
pub mod message;
pub mod roles;

pub const HALF_OF_VOTERS: u8 = 1;

#[cfg(test)]
mod tests {
    use crate::message::{Issue, IssueType};

    #[test]
    fn test_issue() {
        let test_issue = Issue::new("".to_owned(), 0, IssueType::Proposal);

        println!("{:#?}", test_issue);
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
