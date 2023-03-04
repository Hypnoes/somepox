use std::{cell::RefCell, collections::HashMap};

use async_trait::async_trait;

use crate::{
    connection::Connection,
    error::GeneralError,
    mail::{Mail, MailBox},
    message::Issue,
};

#[async_trait]
pub trait Roles<Proposal>
where
    Proposal: Clone + From<Vec<u8>> + Into<Vec<u8>>,
{
    fn msg_pipe(&self) -> &Connection;
    fn send_box(&self) -> &MailBox<Proposal>;
    fn recv_box(&self) -> &MailBox<Proposal>;

    fn draft_new(&self, old_proposal: Mail<Proposal>) -> Result<Mail<Proposal>, GeneralError>;

    /**
        第二步：
           1. ☑️ 从邮箱取出消息
           2. ☑️ 处理消息，进行表决
           3. ☑️ 将表决结果投入至发件箱
    */
    fn process_proposal(&self) -> Result<(), GeneralError> {
        let old_mail = self.recv_box().get_mail()?;
        let new_mail = self.draft_new(old_mail)?;
        self.send_box().put_mail(new_mail);
        Ok(())
    }

    /**
        第一步：
            1. ☑️ 接受消息
            2. ☑️ 反序列化消息至 Proposal
            3. ☑️ 将消息投入至收件箱
    */
    async fn rcv_msg(&self) -> Result<(), GeneralError> {
        let new_msg = self.msg_pipe().recv().await?;
        self.recv_box().put_mail(new_msg.into())
    }

    /**
        第三步：
            1. ☑️ 从发件箱取出消息
            2. ☑️ 发送消息
    */
    async fn send_msg(&self) -> Result<(), GeneralError> {
        let mail = self.send_box().get_mail()?;

        for recivers in mail.receivers().iter() {
            self.msg_pipe()
                .send(recivers, mail.content().into())
                .await?;
        }

        Ok(())
    }

    /**
        典型的角色工作流程：

        1. ☑️ 接受消息
        2. ☑️ 反序列化消息至 Proposal
        3. ☑️ 将消息投入至收件箱
        ---
        4. ☑️ 从邮箱取出消息
        5. ☑️ 处理消息，进行表决
        6. ☑️ 将表决结果投入至发件箱
        ---
        7. ☑️ 从发件箱取出消息
        8. ☑️ 发送消息
    */
    async fn do_work(&self) -> Result<(), GeneralError> {
        self.rcv_msg().await?;
        self.process_proposal()?;
        self.send_msg().await?;
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
    count: RefCell<HashMap<String, u8>>,
}

impl President {}

impl Roles<Issue> for President {
    fn msg_pipe(&self) -> &Connection {
        todo!()
    }

    fn send_box(&self) -> &MailBox<Issue> {
        todo!()
    }

    fn recv_box(&self) -> &MailBox<Issue> {
        todo!()
    }

    fn draft_new(&self, old_proposal: Mail<Issue>) -> Result<Mail<Issue>, GeneralError> {
        todo!()
    }
}

/// 书记：
/// 将议题决定写入记录中
pub struct Secretary {
    mail_box: MailBox<Issue>,
    connection: Connection,
}

impl Secretary {}

impl Roles<Issue> for Secretary {
    fn msg_pipe(&self) -> &Connection {
        todo!()
    }

    fn send_box(&self) -> &MailBox<Issue> {
        todo!()
    }

    fn recv_box(&self) -> &MailBox<Issue> {
        todo!()
    }

    fn draft_new(&self, old_proposal: Mail<Issue>) -> Result<Mail<Issue>, GeneralError> {
        todo!()
    }
}

/// 议员：
/// 1. 对议长下发的议题进行投票：
/// 2. 如果当前消息的id大于已处理的id，同意；否则拒绝
/// 3. 回复投票结果
pub struct Senator {}

impl Roles<Issue> for Senator {
    fn msg_pipe(&self) -> &Connection {
        todo!()
    }

    fn send_box(&self) -> &MailBox<Issue> {
        todo!()
    }

    fn recv_box(&self) -> &MailBox<Issue> {
        todo!()
    }

    fn draft_new(&self, old_proposal: Mail<Issue>) -> Result<Mail<Issue>, GeneralError> {
        todo!()
    }
}

/// 提案者：
/// 提交 议案 至 议长，由议长添加进待议列表
pub struct Proposer {}

impl Roles<Issue> for Proposer {
    fn msg_pipe(&self) -> &Connection {
        todo!()
    }

    fn send_box(&self) -> &MailBox<Issue> {
        todo!()
    }

    fn recv_box(&self) -> &MailBox<Issue> {
        todo!()
    }

    fn draft_new(&self, old_proposal: Mail<Issue>) -> Result<Mail<Issue>, GeneralError> {
        todo!()
    }
}
