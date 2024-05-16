use crate::{
    connection::{Connection, Net},
    mail::{Mail, MailBox},
};
use anyhow::Result;
use bytes::Bytes;
use std::{
    collections::HashMap,
    marker::{Send, Sync},
};

mod president;
mod proposer;
mod secretary;
mod senator;

pub use crate::roles::president::President;
pub use crate::roles::proposer::Proposer;
pub use crate::roles::secretary::Secretary;
pub use crate::roles::senator::Senator;

/*
 通信录：ID -> 地址列表
*/
type AddressBook = HashMap<String, Vec<String>>;

// actor 之间应该怎么通信：
// 1. 线程：通过变量通信
// 2. 进程：通过IPC通信
// 3. 网络：通过Socket通信
pub trait Actor<ActorMessage>
where
    ActorMessage: Clone + TryFrom<Bytes> + Into<Bytes> + Send + Sync,
{
    fn address(&self) -> &String;
    fn address_book(&self) -> &AddressBook;
    fn msg_pipe(&self) -> &Net;
    fn send_box(&self) -> &MailBox<ActorMessage>;
    fn recv_box(&self) -> &MailBox<ActorMessage>;

    fn process(&self, old_message: Mail<ActorMessage>) -> Result<Mail<ActorMessage>>;

    /**
     *  典型的角色工作流程：

        1. 接收阶段：
        接受消息 ➡️ 反序列化消息至 ActorMessage ➡️ 将消息投入至收件箱

        2. 处理阶段：
        从邮箱取出消息 ➡️ 处理消息，进行表决 ➡️ 将表决结果投入至发件箱

        3. 发送阶段：
        从发件箱取出消息 ➡️ 发送消息
    */
    fn invoke(&self) -> Result<()> {
        // 第一步：
        //    1. ☑️ 接受消息
        //    2. ☑️ 反序列化消息至 Proposal
        //    3. ☑️ 将消息投入至收件箱
        let new_msg = self.msg_pipe().recv()?;
        self.recv_box().put_mail(new_msg.try_into()?)?;

        // 第二步：
        //    1. ☑️ 从邮箱取出消息
        //    2. ☑️ 处理消息，进行表决
        //    3. ☑️ 将表决结果投入至发件箱
        let old_mail = self.recv_box().get_mail()?;
        let new_mail = self.process(old_mail)?;
        self.send_box().put_mail(new_mail)?;

        // 第三步：
        //    1. ☑️ 从发件箱取出消息
        //    2. ☑️ 发送消息
        let mail = self.send_box().get_mail()?;

        for recivers in mail.receivers().iter() {
            self.msg_pipe()
                .send(recivers.to_string(), mail.body().into())?;
        }

        Ok(())
    }
}

trait Roles<Proposal>: Actor<Proposal>
where
    Proposal: Clone + TryFrom<Bytes> + Into<Bytes> + Send + Sync,
{
    /*
      帮助函数：通过地址确定发件人身份
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
