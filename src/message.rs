use std::fmt::Display;

use bytes::Bytes;
use log::error;

#[derive(Debug, Clone)]
pub struct Issue {
    content: String,
    id: u32,
    issue_type: IssueType,
}

impl Issue {
    pub fn new(content: String, id: u32, issue_type: IssueType) -> Self {
        Issue {
            content,
            id,
            issue_type,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn issue_type(&self) -> &IssueType {
        &self.issue_type
    }
}

impl From<Bytes> for Issue {
    fn from(bytes: Bytes) -> Self {
        let formatted_issue_string = String::from_utf8(bytes.to_vec());

        match formatted_issue_string {
            Ok(raw_issue_content) => {
                let parts: Vec<&str> = raw_issue_content.split("|").collect();
                let it = IssueType::from(parts[0]);
                let id = u32::from_str_radix(parts[1], 10).unwrap_or(0);
                let ct = parts[2].to_string();

                Issue {
                    content: ct,
                    id: id,
                    issue_type: it,
                }
            }
            Err(_) => {
                error!("not a valid UTF-8 string");
                Issue {
                    content: "".to_owned(),
                    id: 0,
                    issue_type: IssueType::Proposal,
                }
            }
        }
    }
}

impl Into<Bytes> for Issue {
    fn into(self) -> Bytes {
        // 我麻了，深拷贝三次。。。
        let coded_issue_type = self.clone().issue_type.to_string();
        let coded_issue_id = self.clone().id.to_string();
        let coded_issue_content = self.clone().content;

        vec![coded_issue_type, coded_issue_id, coded_issue_content]
            .join("|")
            .into_bytes()
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum IssueType {
    Proposal,
    Vote,
    Resolution,
}

impl Display for IssueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueType::Proposal => write!(f, "p"),
            IssueType::Vote => write!(f, "v"),
            IssueType::Resolution => write!(f, "r"),
        }
    }
}

impl Into<String> for IssueType {
    fn into(self) -> String {
        match self {
            IssueType::Proposal => "p".to_owned(),
            IssueType::Vote => "v".to_owned(),
            IssueType::Resolution => "r".to_owned(),
        }
    }
}

impl From<&str> for IssueType {
    fn from(s: &str) -> Self {
        match s {
            "r" => IssueType::Resolution,
            "v" => IssueType::Vote,
            "p" => IssueType::Proposal,
            _ => panic!("not a valid issue_type"),
        }
    }
}

impl From<String> for IssueType {
    fn from(s: String) -> Self {
        IssueType::from(s.as_str())
    }
}
