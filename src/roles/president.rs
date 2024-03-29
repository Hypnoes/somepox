use std::{cell::RefCell, collections::HashMap};

use crate::{
    connection::{Connection, HostAndPort},
    mail::{Mail, MailBox},
    message::{Issue, IssueType},
    HALF_OF_VOTERS,
};
use anyhow::{anyhow, ensure, Result};

use super::{Roles, PRESIDENT_ROLE_NAME};

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
    count: RefCell<HashMap<u32, u8>>,
}

impl President {
    pub fn new(address_book: HashMap<String, Vec<String>>, endpoint: HostAndPort) -> Self {
        Self {
            address_book,
            send_box: MailBox::new(),
            recv_box: MailBox::new(),
            connection: Connection::new(endpoint),
            count: RefCell::new(HashMap::new()),
        }
    }

    fn my_address() -> String {
        PRESIDENT_ROLE_NAME.to_string()
    }

    // 当收到提案时，生成提案表决记录，并且将提案分发至所有议员
    fn process_proposal(&self, issue: Issue) -> Result<Issue> {
        self.count.borrow_mut().insert(issue.id(), 0);
        Ok(issue)
    }

    // 当收到投票时，为对应议案进行计票，如果票数过半，就生成议案交由书记记录
    // 当投票未过半，返回Error("not enough votes")
    fn process_vote(&self, issue: Issue) -> Result<Issue> {
        let mut cnt_table = self.count.borrow_mut();
        let current_issue_cnt = cnt_table.get(&issue.id());
        match current_issue_cnt {
            // 此决议正在表决中
            Some(cnt) => {
                if cnt + 1 > HALF_OF_VOTERS {
                    cnt_table.remove(&issue.id());
                    Ok(Issue::new(
                        issue.content().to_string(),
                        issue.id(),
                        IssueType::Resolution,
                    ))
                } else {
                    Err(anyhow!("not enough votes"))
                }
            }
            // 此决议已完成表决，或未有此决议的提案
            None => Err(anyhow!(
                "this proposal is either not emmitted or is finished"
            )),
        }
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

    fn draft_new(&self, old_proposal: Mail<Issue>) -> Result<Mail<Issue>> {
        let role = self
            .roles(old_proposal.sender())
            .unwrap_or("error".to_string());

        match old_proposal.body().issue_type() {
            // 将提案分发至所有议员进行表决
            IssueType::Proposal => {
                ensure!(
                    role == "proposer".to_owned(),
                    "recv a `Proposal` from {}",
                    role
                );

                Ok(Mail::new(
                    President::my_address(),
                    self.address_book
                        .get("senator")
                        .map(|senators| senators.join(","))
                        .unwrap_or("".to_string()),
                    self.process_proposal(old_proposal.body())?,
                ))
            }

            // 将表决结果进行计票，超过半数则通过决议交由书记记录
            // NOTE: 当机票结果未过半时，会产生 GeneralError("not enough votes") 以此判断是否产生决议。
            IssueType::Vote => {
                ensure!(role == "senator".to_owned(), "recv a `Vote` from {}", role);

                Ok(Mail::new(
                    President::my_address(),
                    self.address_book
                        .get("secretary")
                        .map(|secretaries| secretaries.join(","))
                        .unwrap_or("".to_string()),
                    self.process_vote(old_proposal.body())?,
                ))
            }

            IssueType::Resolution => {
                log::warn!("President should not process Resolution, `DROP`");
                Err(anyhow!("President should not process Resolution"))
            }
        }
    }
}
