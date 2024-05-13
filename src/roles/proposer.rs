use super::{AddressBook, Roles, PROPOSER_ROLE_NAME};
use crate::{
    connection::{Connection, HostAndPort},
    issue::{Issue, IssueType},
    mail::{Mail, MailBox},
};
use anyhow::{anyhow, Ok, Result};

/// 提案者：
/// 提交 *议案(Proposal)* 至 *议长(President)* ，由 *议长* 添加进待议列表
pub struct Proposer {
    address_book: AddressBook,
    send_box: MailBox<Issue>,
    recv_box: MailBox<Issue>,
    connection: Connection,
    counter: u32,
}

impl Proposer {
    pub fn new(address_book: AddressBook, endpoint: HostAndPort) -> Self {
        Self {
            address_book,
            send_box: MailBox::new(),
            recv_box: MailBox::new(),
            connection: Connection::new(endpoint),
            counter: 0,
        }
    }

    fn my_address() -> String {
        PROPOSER_ROLE_NAME.to_string()
    }
}

impl Roles<Issue> for Proposer {
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

    fn draft_new(&self, _: Mail<Issue>) -> Result<Mail<Issue>> {
        Err(anyhow!("Proposer should always draft new proposor."))
    }

    async fn do_work(&self) -> Result<()> {
        let mail = Mail::new(
            Proposer::my_address(),
            self.address_book()
                .get("president")
                .map(|addr| addr.join(","))
                .unwrap_or("".to_string()),
            Issue::new("todo".to_string(), self.counter + 1, IssueType::Proposal),
        );

        for recivers in mail.receivers().iter() {
            self.msg_pipe().send(recivers, mail.body().into()).await?;
        }

        Ok(())
    }
}
