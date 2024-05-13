use super::{AddressBook, Roles};
use crate::{
    connection::{Connection, HostAndPort},
    issue::{Issue, IssueType},
    logbackend::Writable,
    mail::{Mail, MailBox},
};
use anyhow::{anyhow, Result};
use std::collections::HashMap;

/// 书记：
/// 将议题 *决定(Resolution)* 写入记录中
pub struct Secretary<LogBackend>
where
    LogBackend: Writable,
{
    address_book: AddressBook,
    send_box: MailBox<Issue>,
    recv_box: MailBox<Issue>,
    connection: Connection,
    log_backend: LogBackend,
}

impl<LogBackend: Writable> Secretary<LogBackend> {
    pub fn new(address_book: AddressBook, endpoint: HostAndPort, log_backend: LogBackend) -> Self {
        Secretary {
            address_book,
            send_box: MailBox::new(),
            recv_box: MailBox::new(),
            connection: Connection::new(endpoint),
            log_backend,
        }
    }
}

impl<LogBackend: Writable> Roles<Issue> for Secretary<LogBackend> {
    fn address_book(&self) -> &AddressBook {
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

    fn draft_new(&self, old_proposal: Mail<Issue>) -> Result<Mail<Issue>> {
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
                    Ok(old_proposal)
                } else {
                    Err(anyhow!("recv a `Resolution` from {}", role))
                }
            }
        }
    }

    async fn do_work(&self) -> Result<()> {
        let new_msg = self.msg_pipe().recv().await?;
        self.recv_box().put_mail(new_msg.try_into()?)?;
        self.draft_new(self.recv_box().get_mail()?)?;
        Ok(())
    }
}
