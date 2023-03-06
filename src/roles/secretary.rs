use std::collections::HashMap;

use crate::{
    connection::Connection,
    error::GeneralError,
    logbackend::Writable,
    mail::{Mail, MailBox},
    message::{Issue, IssueType},
};

use super::Roles;

/// 书记：
/// 将议题 *决定(Resolution)* 写入记录中
pub struct Secretary<LogBackend>
where
    LogBackend: Writable,
{
    address_book: HashMap<String, Vec<String>>,
    send_box: MailBox<Issue>,
    recv_box: MailBox<Issue>,
    connection: Connection,
    log_backend: LogBackend,
}

impl<LogBackend: Writable> Secretary<LogBackend> {
    fn write_to_log(&self, issue: Issue) -> Result<(), GeneralError> {
        self.log_backend.write(issue.into())
    }
}

impl<LogBackend: Writable> Roles<Issue> for Secretary<LogBackend> {
    fn address_book(&self) -> &HashMap<String, Vec<String>> {
        &(self.address_book)
    }

    fn msg_pipe(&self) -> &Connection {
        &(self.connection)
    }

    fn send_box(&self) -> &MailBox<Issue> {
        &(self.send_box)
    }

    fn recv_box(&self) -> &MailBox<Issue> {
        &(self.recv_box)
    }

    fn draft_new(&self, old_proposal: Mail<Issue>) -> Result<Mail<Issue>, GeneralError> {
        let role = self
            .roles(old_proposal.sender())
            .unwrap_or("error".to_string());

        match old_proposal.body().issue_type() {
            IssueType::Proposal => {
                log::warn!("Secretary should not process proposals, `DROP`");
                Err("Secretary should not process proposals".to_string().into())
            }
            IssueType::Vote => {
                log::warn!("Secretary should not process votes, `DROP`");
                Err("Secretary should not process votes".to_string().into())
            }
            IssueType::Resolution => {
                if role == "president".to_owned() {
                    self.write_to_log(old_proposal.body())?;
                    Ok(old_proposal)
                } else {
                    Err(format!("recv a `Resolution` from {}", role).into())
                }
            }
        }
    }
}