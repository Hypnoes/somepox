use std::collections::HashMap;

use super::AddressBook;
use crate::{
    connection::Net,
    issue::{Issue, IssueType},
    logbackend::{Queryable, Writable},
    mail::{Mail, MailBox},
};
use anyhow::{anyhow, Result};

/// 书记：
/// 将议题 *决定(Resolution)* 写入记录中
pub struct Secretary<LogBackend>
where
    LogBackend: Writable + Queryable,
{
    address: String,
    address_book: AddressBook,
    send_box: MailBox<Issue>,
    recv_box: MailBox<Issue>,
    connection: Net,
    log_backend: LogBackend,
}

impl<LogBackend: Writable + Queryable> Secretary<LogBackend> {
    pub fn new(
        address: String,
        address_book: AddressBook,
        log_backend: LogBackend,
    ) -> Result<Self> {
        let conn = Net::new(address.clone())?;
        Ok(Self {
            address: address,
            address_book: address_book,
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

    pub fn get_log(&self, id: u64) -> Result<String> {
        let query_result = self.log_backend.query(id)?;
        Ok(String::from_utf8(query_result.into())?)
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
