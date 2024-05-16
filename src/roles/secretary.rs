use std::collections::HashMap;

use super::AddressBook;
use crate::{
    connection::Net,
    issue::{Issue, IssueType},
    logbackend::Writable,
    mail::{Mail, MailBox},
};
use anyhow::{anyhow, Result};

/// 书记：
/// 将议题 *决定(Resolution)* 写入记录中
pub struct Secretary<LogBackend>
where
    LogBackend: Writable,
{
    address: String,
    address_book: AddressBook,
    send_box: MailBox<Issue>,
    recv_box: MailBox<Issue>,
    connection: Net,
    log_backend: LogBackend,
}

impl<LogBackend: Writable> Secretary<LogBackend> {
    pub fn new(address: String, log_backend: LogBackend) -> Result<Self> {
        let conn = Net::new(address.as_str())?;
        Ok(Self {
            address: address,
            address_book: HashMap::new(),
            send_box: MailBox::new(),
            recv_box: MailBox::new(),
            connection: conn,
            log_backend,
        })
    }

    pub fn log_issue(&self, old_proposal: Mail<Issue>) -> Result<()> {
        let write_to_log = |issue: Issue| {
            self.log_backend
                .write(issue.id().into(), issue.content().into())
        };

        let role = self
            .roles(old_proposal.sender())
            .unwrap_or("error".to_string());

        match old_proposal.body().issue_type() {
            IssueType::Proposal => {
                log::warn!("Secretary should not process proposals, `DROP`");
                Err(anyhow!("Secretary should not process proposals"))
            }
            IssueType::Vote => {
                log::warn!("Secretary should not process votes, `DROP`");
                Err(anyhow!("Secretary should not process votes"))
            }
            IssueType::Resolution => {
                if &role == "president" {
                    write_to_log(old_proposal.body())?;
                    Ok(())
                } else {
                    Err(anyhow!("recv a `Resolution` from {}", role))
                }
            }
        }
    }

    fn roles(&self, address: String) -> Option<String> {
        for (role_id, role_address) in self.address_book.iter() {
            if role_address.contains(&address) {
                return Some(role_id.clone());
            }
        }
        None
    }
}
