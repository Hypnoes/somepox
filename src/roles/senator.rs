use std::collections::HashMap;

use crate::{
    connection::Connection,
    error::GeneralError,
    mail::{Mail, MailBox},
    message::Issue,
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
        todo!()
    }
}
