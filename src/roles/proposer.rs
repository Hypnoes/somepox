use std::collections::HashMap;

use crate::{
    connection::Connection,
    error::GeneralError,
    mail::{Mail, MailBox},
    message::{Issue, IssueType},
};

use super::Roles;

/// 提案者：
/// 提交 *议案(Proposal)* 至 *议长(President)* ，由 *议长* 添加进待议列表
pub struct Proposer {
    address_book: HashMap<String, Vec<String>>,
    send_box: MailBox<Issue>,
    recv_box: MailBox<Issue>,
    connection: Connection,
    counter: u32,
}

impl Proposer {
    fn my_address() -> String {
        String::from("0x0000000000000000000000000000000000000000")
    }
}

impl Roles<Issue> for Proposer {
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
        Ok(Mail::new(
            Proposer::my_address(),
            self.address_book()
                .get("president")
                .map(|addr| addr.join(","))
                .unwrap_or("".to_string()),
            Issue::new(
                old_proposal.body().content().to_string(),
                self.counter + 1,
                IssueType::Proposal,
            ),
        ))
    }
}
