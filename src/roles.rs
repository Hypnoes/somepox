use std::{collections::HashMap, error::Error};

use log::warn;
use tokio::runtime::Builder;

use crate::{
    connection::Connection,
    mail::MailBox,
    message::{Issue, IssueType},
};

const HALF_OF_VOTERS: u8 = 1;

pub trait Roles<'a, Proposal>
where
    Proposal: From<Vec<u8>>,
    &'a Proposal: Into<Vec<u8>> + 'a,
{
    fn connection(&self) -> &Connection;
    fn mailbox(&self) -> &MailBox<Proposal>;
    fn handle_this_doc(&self, doc: &'a Proposal) -> Result<(), Box<dyn Error>>;

    fn do_work(&mut self) -> Result<(), Box<dyn Error>> {
        let doc = self.mailbox().get();

        match doc {
            Some(d) => self.handle_this_doc(d.content()),
            None => Ok(()),
        }
    }

    fn send_msg(&self, msg: &'a Proposal, to: &str) -> Result<(), Box<dyn Error>> {
        let conn = self.connection();
        let rt = Builder::new_current_thread()
            .enable_all()
            .worker_threads(1)
            .max_blocking_threads(1)
            .build()?;

        let send_list = self.mailbox().address_book().iter().filter(|x| x.0 == to);

        for x in send_list {
            rt.block_on(conn.send(&(x.1), msg.into()));
        }

        Ok(())
    }
}

/// 议长：
/// 1. 从 *议员(Proposer)* 接受 *提案(Issue | Proposol)*
/// 2. 将 *提案(Issue | Proposol)* 交由所有 *议员(Senator)* *表决(Vote)*
/// 3. 将 *表决(Vote)* 结果收回，*唱票(Counting)*
/// 4. 将投票结果交由 *书记(Secretary)* 记录在案形成最终 *决议(Log | Resolution)*
pub struct President {
    mail_box: MailBox<Issue>,
    connection: Connection,
    count: HashMap<String, u8>,
}

impl President {
    fn hand_out_issue<'a>(&mut self, doc: &'a Issue) -> Result<(), Box<dyn Error>> {
        self.count.insert(doc.id().to_string(), 0);
        self.send_msg(doc, "senator")
    }
    fn count_vote<'a>(&mut self, doc: &'a Issue) -> Result<(), Box<dyn Error>> {
        let vote_result = self.count.get(doc.id());
        if vote_result
            .filter(move |vote_count| **vote_count > HALF_OF_VOTERS)
            .is_some()
        {
            self.send_msg(doc, "secretary");
        }
        Ok(())
    }
}

impl<'a> Roles<'a, Issue> for President {
    fn connection(&self) -> &Connection {
        &self.connection
    }

    fn mailbox(&self) -> &MailBox<Issue> {
        &self.mail_box
    }

    fn handle_this_doc(&self, doc: &'a Issue) -> Result<(), Box<dyn Error>> {
        match doc.issue_type() {
            IssueType::Proposal => self.hand_out_issue(doc),
            IssueType::Vote => self.count_vote(doc),
            IssueType::Log => {
                warn!("Receive wrong message: {:?}, Drop.", doc);
                Ok(())
            }
        }
    }
}

/// 书记：
/// 将议题决定写入记录中
pub struct Secretary {
    mail_box: MailBox<Issue>,
    connection: Connection,
}

impl Secretary {
    fn write_to_memo(&self, doc: &Issue) {}
}

impl<'a> Roles<'a, Issue> for Secretary {
    fn connection(&self) -> &Connection {
        &self.connection
    }

    fn mailbox(&self) -> &MailBox<Issue> {
        &self.mail_box
    }

    fn handle_this_doc(&self, doc: &'a Issue) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

/// 议员：
/// 1. 对议长下发的议题进行投票：
/// 2. 如果当前消息的id大于已处理的id，同意；否则拒绝
/// 3. 回复投票结果
pub struct Senator {
    mail_box: MailBox<Issue>,
}

/// 提案者：
/// 提交 议案 至 议长，由议长添加进待议列表
pub struct Proposer {
    mail_box: MailBox<Issue>,
}
