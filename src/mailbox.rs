#![allow(unused)]

use std::{cell::RefCell, collections::VecDeque};

use anyhow::{anyhow, Result};
use bytes::Bytes;

use crate::connection::Connection;

pub struct MailBox<Content>
    where
        Content: Clone + TryFrom<Bytes> + Into<Bytes>,
{
    send_list: RefCell<VecDeque<Mail<Content>>>,
    recv_list: RefCell<VecDeque<Mail<Content>>>,
    conn: Box<dyn Connection<Addr=String>>,
}

impl<Message> MailBox<Message>
    where
        Message: Clone + TryFrom<Bytes> + Into<Bytes>,
{
    pub fn new(conn: Box<dyn Connection<Addr=String>>) -> Self {
        MailBox {
            send_list: RefCell::new(VecDeque::new()),
            recv_list: RefCell::new(VecDeque::new()),
            conn: conn,
        }
    }

    /// 从收件箱获取新邮件
    pub fn get_mail(&self) -> Result<Mail<Message>> {
        match self.recv_list.try_borrow_mut()?.pop_front() {
            Some(mail) => Ok(mail),
            None => Err(anyhow!("MailBox is empty")),
        }
    }

    /// 将邮件放置入发件箱
    pub fn put_mail(&self, mail: Mail<Message>) -> Result<()> {
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

pub struct Mail<Content>
    where
        Content: Clone + TryFrom<Bytes> + Into<Bytes>,
{
    from: String,
    to: String,
    body: Box<Content>,
}

impl<Content: Clone + TryFrom<Bytes> + Into<Bytes>> Mail<Content> {
    pub fn new(from: String, to: String, content: Content) -> Mail<Content> {
        Mail {
            from,
            to,
            body: Box::new(content),
        }
    }

    pub fn body(&self) -> Content {
        (*self.body).clone()
    }

    pub fn sender(&self) -> String {
        self.from.clone()
    }

    pub fn receivers(&self) -> Vec<String> {
        let mut rcs_v: Vec<String> = Vec::new();

        for rcs in self.to.split(",").into_iter() {
            rcs_v.push(rcs.to_string());
        }

        rcs_v
    }
}

impl<A, Content> TryFrom<(A, A, Bytes)> for Mail<Content>
    where
        Content: Clone + TryFrom<Bytes> + Into<Bytes>,
        A: TryFrom<String> + Into<String>,
{
    type Error = anyhow::Error;

    fn try_from(value: (A, A, Bytes)) -> std::result::Result<Self, Self::Error> {
        Ok(Mail::new(
            value.0.into(),
            value.1.into(),
            Content::try_from(value.2)
                .map_err(|_| anyhow!("can not deserialize bytes into Main::Content"))?,
        ))
    }
}
