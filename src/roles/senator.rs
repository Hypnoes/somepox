use std::collections::HashMap;

use super::{Actor, AddressBook, Roles};
use crate::{
    connection::Net,
    issue::{Issue, IssueType},
    mail::{Mail, MailBox},
};
use anyhow::{anyhow, Result};

/// 议员：
/// 1. 对 *议长(President)* 下发的 *议题(Proposal)* 进行投票
/// 2. 如果当前议题的 *编号(id)* 大于已处理的 *编号(id)* ，同意该提案；否则拒绝
/// 3. 回复投票结果至 *议长(President)*
pub struct Senator {
    address: String,
    address_book: AddressBook,
    send_box: MailBox<Issue>,
    recv_box: MailBox<Issue>,
    connection: Net,
    last_proposal_id: u32,
}

impl Senator {
    pub fn new(address: String) -> Result<Self> {
        let conn = Net::new(address.as_str())?;
        Ok(Self {
            address: address,
            address_book: HashMap::new(),
            send_box: MailBox::new(),
            recv_box: MailBox::new(),
            connection: conn,
            last_proposal_id: 0,
        })
    }
}

impl Actor<Issue> for Senator {
    fn address(&self) -> &String {
        &(self.address)
    }

    fn address_book(&self) -> &AddressBook {
        &(self.address_book)
    }

    fn msg_pipe(&self) -> &Net {
        &(self.connection)
    }

    fn send_box(&self) -> &MailBox<Issue> {
        &(self.send_box)
    }

    fn recv_box(&self) -> &MailBox<Issue> {
        &(self.recv_box)
    }

    fn process(&self, old_proposal: Mail<Issue>) -> Result<Mail<Issue>> {
        let role = self
            .roles(old_proposal.sender())
            .unwrap_or("error".to_string());

        match old_proposal.body().issue_type() {
            IssueType::Proposal => {
                if role == "president".to_string() {
                    if old_proposal.body().id() > self.last_proposal_id {
                        Ok(Mail::new(
                            self.address.clone(),
                            self.address_book()
                                .get("president")
                                .map(|addr| addr.join(","))
                                .unwrap_or("".to_string()),
                            Issue::new(
                                old_proposal.body().content().to_string(),
                                old_proposal.body().id(),
                                IssueType::Vote,
                            ),
                        ))
                    } else {
                        Err(anyhow!(
                            "received expire issue {}, last issue is {}, drop.",
                            old_proposal.body().id(),
                            self.last_proposal_id
                        ))
                    }
                } else {
                    Err(anyhow!("recv a `Proposal` from {}", role))
                }
            }
            IssueType::Vote => Err(anyhow!("Senator does not process issue: Vote")),
            IssueType::Resolution => Err(anyhow!("Senator does not process issue: Vote")),
        }
    }
}

impl Roles<Issue> for Senator {}
