use std::{cell::RefCell, collections::HashMap};

use crate::{
    connection::Connection,
    error::GeneralError,
    mail::{Mail, MailBox},
    message::{Issue, IssueType},
};

use super::Roles;

/// 议长：
/// 1. 从 *提议者(Proposer)* 接受 *提案(Proposol)*
/// 2. 将 *提案(Proposol)* 交由所有 *议员(Senator)* *表决(Vote)*
/// 3. 将 *表决(Vote)* 结果收回，*唱票(Counting)*
/// 4. 将投票结果交由 *书记(Secretary)* 记录在案形成最终 *决议(Resolution)*
pub struct President {
    address_book: HashMap<String, Vec<String>>,
    send_box: MailBox<Issue>,
    recv_box: MailBox<Issue>,
    connection: Connection,
    count: RefCell<HashMap<String, u8>>,
}

impl President {
    fn my_address() -> String {
        "persident".to_string()
    }

    fn process_proposal(&self, issue: Issue) -> Result<Issue, GeneralError> {
        todo!()
    }
    fn process_vote(&self, issue: Issue) -> Result<Issue, GeneralError> {
        todo!()
    }
}

impl Roles<Issue> for President {
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
            // 将提案分发至所有议员进行表决
            IssueType::Proposal => {
                if role == "proposer".to_owned() {
                    let new_mail = Mail::new(
                        President::my_address(),
                        self.address_book
                            .get("senator")
                            .map(|senators| senators.join(","))
                            .unwrap_or("".to_string()),
                        self.process_proposal(old_proposal.body())?,
                    );
                    Ok(new_mail)
                } else {
                    Err(format!("recv a `Proposal` from {}", role).into())
                }
            }

            // 将表决结果进行计票，超过半数则通过决议交由书记记录
            // NOTE: 当机票结果未过半时，会产生 GeneralError("not enough votes") 以此判断是否产生决议。
            IssueType::Vote => {
                if role == "senator".to_owned() {
                    let new_mail = Mail::new(
                        President::my_address(),
                        self.address_book
                            .get("secretary")
                            .map(|secretaries| secretaries.join(","))
                            .unwrap_or("".to_string()),
                        self.process_vote(old_proposal.body())?,
                    );
                    Ok(new_mail)
                } else {
                    Err(format!("recv a `Vote` from {}", role).into())
                }
            }

            IssueType::Resolution => {
                log::warn!("President should not process Resolution, `DROP`");
                Err("President should not process Resolution".to_string().into())
            }
        }
    }
}
