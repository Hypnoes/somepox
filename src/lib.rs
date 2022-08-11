pub mod connection;
pub mod mail;
pub mod message;
pub mod roles;

#[cfg(test)]
mod tests {
    use crate::message::{Issue, IssueType};

    #[test]
    fn test_issue() {
        let test_issue = Issue::new("test".to_string(), IssueType::Log);

        println!("{:#?}", test_issue);
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
