use std::fmt::Display;

use bytes::Bytes;

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

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn id(&self) -> u32 {
        self.id.clone()
    }

    pub fn issue_type(&self) -> IssueType {
        self.issue_type.clone()
    }
}

impl TryFrom<Bytes> for Issue {
    type Error = anyhow::Error;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        let raw_issue_content = String::from_utf8(value.to_vec())
            .map_err(|_| anyhow::anyhow!("Failed to convert bytes to string"))?;

        let parts: Vec<&str> = raw_issue_content.split("|").collect();

        if parts.len() != 3 {
            return Err(anyhow::anyhow!(
                "Invalid issue format, expected 3 parts, got {}",
                parts.len()
            ));
        } else {
            let parts: Vec<&str> = raw_issue_content.split("|").collect();

            let it = IssueType::try_from(parts[0])?;
            let id = u32::from_str_radix(parts[1], 10)?;
            let ct = parts[2];

            Ok(Issue {
                content: ct.to_owned(),
                id: id,
                issue_type: it,
            })
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

impl TryFrom<&str> for IssueType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "r" => Ok(IssueType::Resolution),
            "v" => Ok(IssueType::Vote),
            "p" => Ok(IssueType::Proposal),
            _ => Err(anyhow::anyhow!("not a valid issue_type")),
        }
    }
}
impl TryFrom<String> for IssueType {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        IssueType::try_from(value.as_str())
    }
}
