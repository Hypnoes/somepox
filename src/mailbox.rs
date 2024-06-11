#![allow(unused)]

use std::{cell::RefCell, collections::VecDeque};

use anyhow::{anyhow, Result};
use bytes::Bytes;

use crate::connection::Connection;

pub struct MailBox<Addr, Content>
where
    Addr: Clone,
    Content: Clone + TryFrom<Bytes> + Into<Bytes>,
{
    send_list: RefCell<VecDeque<Mail<Addr, Content>>>,
    recv_list: RefCell<VecDeque<Mail<Addr, Content>>>,
    conn: Box<dyn Connection<Addr = Addr>>,
}

impl<Addr, Content> MailBox<Addr, Content>
where
    Addr: Clone,
    Content: Clone + TryFrom<Bytes> + Into<Bytes>,
{
    pub fn new(conn: Box<dyn Connection<Addr = Addr>>) -> Self {
        MailBox {
            send_list: RefCell::new(VecDeque::new()),
            recv_list: RefCell::new(VecDeque::new()),
            conn: conn,
        }
    }

    /// 从收件箱获取新邮件
    pub fn get_mail(&self) -> Result<Mail<Addr, Content>> {
        match self.recv_list.try_borrow_mut()?.pop_front() {
            Some(mail) => Ok(mail),
            None => Err(anyhow!("MailBox is empty")),
        }
    }

    /// 将邮件放置入发件箱
    pub fn put_mail(&self, mail: Mail<Addr, Content>) -> Result<()> {
        Ok(self.send_list.try_borrow_mut()?.push_back(mail))
    }

    /// 将所有发件箱中的待发邮件发送至接收者。
    pub fn flush(&self) -> Result<()> {
        for mail in self.send_list.borrow_mut().iter() {
            for receivers in mail.receivers().iter() {
                self.conn
                    .send(receivers.to_owned().into(), mail.body().into())?;
            }
        }
        Ok(())
    }

    /// Block 阻塞直至收到新邮件，并添加至收件箱。
    pub fn fill_msg_box(&self) -> Result<()> {
        let mail = self.conn.recv()?;
        Ok(self.recv_list.try_borrow_mut()?.push_back(mail.try_into()?))
    }
}

pub struct Mail<Addr, Content>
where
    Addr: Clone,
    Content: Clone + TryFrom<Bytes> + Into<Bytes>,
{
    from: Addr,
    to: Vec<Addr>,
    body: Box<Content>,
}

impl<Addr, Content> Mail<Addr, Content>
where
    Addr: Clone,
    Content: Clone + TryFrom<Bytes> + Into<Bytes>,
{
    pub fn new(from: Addr, to: Vec<Addr>, content: Content) -> Mail<Addr, Content> {
        Mail {
            from,
            to,
            body: Box::new(content),
        }
    }

    pub fn body(&self) -> Content {
        (*self.body).clone()
    }

    pub fn sender(&self) -> Addr {
        self.from.clone()
    }

    pub fn receivers(&self) -> Vec<Addr> {
        self.to.clone()
    }
}

impl<Addr, Content> TryFrom<(Addr, Addr, Bytes)> for Mail<Addr, Content>
where
    Addr: Clone,
    Content: Clone + TryFrom<Bytes> + Into<Bytes>,
{
    type Error = anyhow::Error;

    fn try_from(value: (Addr, Addr, Bytes)) -> std::result::Result<Self, Self::Error> {
        Ok(Mail::new(
            value.0,
            vec![value.1],
            Content::try_from(value.2)
                .map_err(|_| anyhow!("can not deserialize bytes into Main::Content"))?,
        ))
    }
}

impl<Addr, Content> TryFrom<(Addr, Vec<Addr>, Bytes)> for Mail<Addr, Content>
where
    Addr: Clone,
    Content: Clone + TryFrom<Bytes> + Into<Bytes>,
{
    type Error = anyhow::Error;

    fn try_from(value: (Addr, Vec<Addr>, Bytes)) -> std::result::Result<Self, Self::Error> {
        Ok(Mail::new(
            value.0,
            value.1,
            Content::try_from(value.2)
                .map_err(|_| anyhow!("can not deserialize bytes into Main::Content"))?,
        ))
    }
}
