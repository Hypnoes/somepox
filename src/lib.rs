pub mod connection;
pub mod error;
pub mod mail;
pub mod message;
pub mod roles;

type Deserializable = dyn From<Vec<u8>>;
type Serializable = dyn Into<Vec<u8>>;

#[cfg(test)]
mod tests {
    use crate::message::{Issue, IssueType};

    #[test]
    fn test_issue() {
        let test_issue = Issue::new("test", IssueType::Log);

        println!("{:#?}", test_issue);
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
