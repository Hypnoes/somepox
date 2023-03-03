use std::{borrow::Borrow, fmt::Display};

use log::error;

#[derive(Debug, Clone)]
pub struct Issue {
    content: String,
    id: u32,
    issue_type: IssueType,
}

impl Issue {
    pub fn new(contents: &str, issue_types: IssueType) -> Issue {
        Issue {
            content: contents.to_string(),
            id: 0,
            issue_type: issue_types,
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

impl From<Vec<u8>> for Issue {
    fn from(bytes: Vec<u8>) -> Self {
        let formatted_issue_string = String::from_utf8(bytes);

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
                Issue::new("", IssueType::Log)
            }
        }
    }
}

impl Into<Vec<u8>> for Issue {
    fn into(self) -> Vec<u8> {
        self.borrow().into()
    }
}

impl Into<Vec<u8>> for &Issue {
    fn into(self) -> Vec<u8> {
        let coded_issue_type = self.issue_type.to_string().as_bytes();
        let coded_issue_id = self.id.to_string().as_bytes();
        let coded_issue_content = self.content.as_bytes();

        vec![coded_issue_type, coded_issue_id, coded_issue_content].join("|".as_bytes())
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
