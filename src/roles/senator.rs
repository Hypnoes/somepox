use std::collections::HashMap;

use crate::{
    connection::Connection,
    error::GeneralError,
    mail::{Mail, MailBox},
    message::{Issue, IssueType},
};

use super::Roles;

/// 议员：
/// 1. 对 *议长(President)* 下发的 *议题(Proposal)* 进行投票
/// 2. 如果当前议题的 *编号(id)* 大于已处理的 *编号(id)* ，同意该提案；否则拒绝
/// 3. 回复投票结果至 *议长(President)*
pub struct Senator {
    address_book: HashMap<String, Vec<String>>,
    send_box: MailBox<Issue>,
    recv_box: MailBox<Issue>,
    connection: Connection,
    last_proposal_id: u32,
}

impl Senator {
    fn my_address() -> String {
        String::from("???")
    }
}

impl Roles<Issue> for Senator {
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
            crate::message::IssueType::Proposal => {
                if role == "president".to_owned() {
                    if old_proposal.body().id() > self.last_proposal_id {
                        Ok(Mail::new(
                            Senator::my_address(),
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
                        Err(format!(
                            "received expire issue {}, last issue is {}, drop.",
                            old_proposal.body().id(),
                            self.last_proposal_id
                        )
                        .into())
                    }
                } else {
                    Err(format!("recv a `Proposal` from {}", role).into())
                }
            }
            crate::message::IssueType::Vote => {
                Err("Senator does not process issue: Vote".to_owned().into())
            }
            crate::message::IssueType::Resolution => {
                Err("Senator does not process issue: Vote".to_owned().into())
            }
        }
    }
}
