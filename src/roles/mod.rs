use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;

use crate::{
    connection::Connection,
    logbackend::Writable,
    mail::{Mail, MailBox},
};

mod president;
mod proposer;
mod secretary;
mod senator;

pub use crate::roles::president::President;
pub use crate::roles::proposer::Proposer;
pub use crate::roles::secretary::Secretary;
pub use crate::roles::senator::Senator;

#[async_trait]
trait Roles<Proposal>
where
    Proposal: Clone + TryFrom<Bytes> + Into<Bytes> + std::marker::Send + std::marker::Sync,
{
    fn address_book(&self) -> &HashMap<String, Vec<String>>;
    fn msg_pipe(&self) -> &Connection;
    fn send_box(&self) -> &MailBox<Proposal>;
    fn recv_box(&self) -> &MailBox<Proposal>;

    fn draft_new(&self, old_proposal: Mail<Proposal>) -> Result<Mail<Proposal>>;

    /**
        第二步：
           1. ☑️ 从邮箱取出消息
           2. ☑️ 处理消息，进行表决
           3. ☑️ 将表决结果投入至发件箱
    */
    fn process_proposal(&self) -> Result<()> {
        let old_mail = self.recv_box().get_mail()?;
        let new_mail = self.draft_new(old_mail)?;
        self.send_box().put_mail(new_mail)?;
        Ok(())
    }

    /**
        第一步：
            1. ☑️ 接受消息
            2. ☑️ 反序列化消息至 Proposal
            3. ☑️ 将消息投入至收件箱
    */
    async fn rcv_msg(&self) -> Result<()> {
        let new_msg = self.msg_pipe().recv().await?;
        self.recv_box().put_mail(new_msg.try_into()?)
    }

    /**
        第三步：
            1. ☑️ 从发件箱取出消息
            2. ☑️ 发送消息
    */
    async fn send_msg(&self) -> Result<()> {
        let mail = self.send_box().get_mail()?;

        for recivers in mail.receivers().iter() {
            self.msg_pipe().send(recivers, mail.body().into()).await?;
        }

        Ok(())
    }

    /**
     * FIXME - 这个建模流程有问题，如 *proposer* 和 *secretary* 这两个角色都不遵循这个流程

        典型的角色工作流程：

        1. 接收阶段：
        接受消息 ➡️ 反序列化消息至 Proposal ➡️ 将消息投入至收件箱

        2. 处理阶段：
        从邮箱取出消息 ➡️ 处理消息，进行表决 ➡️ 将表决结果投入至发件箱

        3. 发送阶段：
        从发件箱取出消息 ➡️ 发送消息
    */
    async fn do_work(&self) -> Result<()> {
        self.rcv_msg().await?;
        self.process_proposal()?;
        self.send_msg().await?;
        Ok(())
    }

    /**
     * 帮助函数：通过地址确定发件人身份
     */
    fn roles(&self, address: String) -> Option<String> {
        for (role_id, role_address) in self.address_book().iter() {
            if role_address.contains(&address) {
                return Some(role_id.clone());
            }
        }
        None
    }
}

pub fn new_president(addr_book: HashMap<String, Vec<String>>, endpoint: &str) -> Result<President> {
    let host_and_port = endpoint.try_into()?;
    Ok(President::new(addr_book, host_and_port))
}

pub fn new_proposer(addr_book: HashMap<String, Vec<String>>, endpoint: &str) -> Result<Proposer> {
    let host_and_port = endpoint.try_into()?;
    Ok(Proposer::new(addr_book, host_and_port))
}

pub fn new_secretary<LogBackend: Writable>(
    addr_book: HashMap<String, Vec<String>>,
    endpoint: &str,
    log_backend: LogBackend,
) -> Result<Secretary<LogBackend>> {
    let host_and_port = endpoint.try_into()?;
    Ok(Secretary::new(addr_book, host_and_port, log_backend))
}

pub fn new_senator(addr_book: HashMap<String, Vec<String>>, endpoint: &str) -> Result<Senator> {
    let host_and_port = endpoint.try_into()?;
    Ok(Senator::new(addr_book, host_and_port))
}

pub const PRESIDENT_ROLE_NAME: &str = "President";
pub const PROPOSER_ROLE_NAME: &str = "Proposer";
pub const SECRETARY_ROLE_NAME: &str = "Secretary";
pub const SENATOR_ROLE_NAME: &str = "Senator";
