use std::{borrow::Borrow, fmt::Display};

use log::error;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Issue {
    content: String,
    id: String,
    issue_type: IssueType,
}

impl Issue {
    pub fn new(contents: String, issue_types: IssueType) -> Issue {
        Issue {
            content: contents,
            id: Uuid::new_v4().to_string(),
            issue_type: issue_types,
        }
    }

    pub fn content(&self) -> &str {
        self.content.borrow()
    }

    pub fn id(&self) -> &str {
        self.id.borrow()
    }

    pub fn issue_type(&self) -> &IssueType {
        self.issue_type.borrow()
    }
}

impl From<Vec<u8>> for Issue {
    fn from(bytes: Vec<u8>) -> Self {
        let s = String::from_utf8(bytes);

        match s {
            Ok(r) => {
                let g: Vec<&str> = r.split("|").collect();
                let it = IssueType::from(g[0]);
                let id = g[1];
                let cc = g[2];

                Issue {
                    content: cc.to_string(),
                    id: id.to_string(),
                    issue_type: it,
                }
            }
            Err(_) => {
                error!("not a valid UTF-8 string");
                Issue::new("".to_string(), IssueType::Log)
            }
        }
    }
}

impl Into<Vec<u8>> for Issue {
    fn into(self) -> Vec<u8> {
        let strf = vec![self.issue_type.to_string(), self.id, self.content];
        strf.join("|").as_bytes().to_vec()
    }
}

#[derive(Debug, Clone)]
pub enum IssueType {
    Proposal,
    Vote,
    Log,
}

impl Display for IssueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueType::Proposal => write!(f, "proposal"),
            IssueType::Vote => write!(f, "vote"),
            IssueType::Log => write!(f, "log"),
        }
    }
}

impl From<&str> for IssueType {
    fn from(s: &str) -> Self {
        match s {
            "l" => IssueType::Log,
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
