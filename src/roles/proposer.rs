use std::collections::HashMap;

use super::AddressBook;
use crate::{
    connection::{Connection, Net},
    issue::{Issue, IssueType},
    mail::{Mail, MailBox},
};
use anyhow::{Ok, Result};

/// 提案者：
/// 提交 *议案(Proposal)* 至 *议长(President)* ，由 *议长* 添加进待议列表
pub struct Proposer {
    address: String,
    address_book: AddressBook,
    send_box: MailBox<Issue>,
    recv_box: MailBox<Issue>,
    connection: Net,
    counter: u32,
}

impl Proposer {
    pub fn new(address: String) -> Result<Self> {
        let conn = Net::new(address.clone())?;
        Ok(Self {
            address: address,
            address_book: HashMap::new(),
            send_box: MailBox::new(),
            recv_box: MailBox::new(),
            connection: conn,
            counter: 0,
        })
    }

    pub fn emmit(&self, msg_content: &str) -> Result<()> {
        let mail = Mail::new(
            self.address.clone(),
            self.address_book
                .get("president")
                .map(|addr| addr.join(","))
                .unwrap_or(msg_content.to_string()),
            Issue::new(
                msg_content.to_string(),
                self.counter + 1,
                IssueType::Proposal,
            ),
        );

        for recivers in mail.receivers().iter() {
            self.connection
                .send(recivers.to_string(), mail.body().into())?;
        }

        Ok(())
    }
}
