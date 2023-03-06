mod president;
mod proposer;
mod secretary;
mod senator;

use std::collections::HashMap;

use async_trait::async_trait;
use bytes::Bytes;

use crate::{
    connection::Connection,
    error::GeneralError,
    mail::{Mail, MailBox},
};

pub use crate::roles::president::President;
pub use crate::roles::proposer::Proposer;
pub use crate::roles::secretary::Secretary;
pub use crate::roles::senator::Senator;

#[async_trait]
pub trait Roles<Proposal>
where
    Proposal: Clone + From<Bytes> + Into<Bytes> + std::marker::Send + std::marker::Sync,
{
    fn address_book(&self) -> &HashMap<String, Vec<String>>;
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
        self.send_box().put_mail(new_mail)?;
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
            self.msg_pipe().send(recivers, mail.body().into()).await?;
        }

        Ok(())
    }

    /**
        典型的角色工作流程：

        1. 接收阶段：
        1-1. ☑️ 接受消息
        1-2. ☑️ 反序列化消息至 Proposal
        1-3. ☑️ 将消息投入至收件箱

        2. 处理阶段：
        2-1. ☑️ 从邮箱取出消息
        2-2. ☑️ 处理消息，进行表决
        2-3. ☑️ 将表决结果投入至发件箱

        3. 发送阶段：
        3-1. ☑️ 从发件箱取出消息
        3-2. ☑️ 发送消息
    */
    async fn do_work(&self) -> Result<(), GeneralError> {
        self.rcv_msg().await?;
        self.process_proposal()?;
        self.send_msg().await?;
        Ok(())
    }

    fn roles(&self, address: String) -> Option<String> {
        for (role_id, role_address) in self.address_book().iter() {
            if role_address.contains(&address) {
                return Some(role_id.clone());
            }
        }
        None
    }
}
