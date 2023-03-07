use std::{cell::RefCell, collections::VecDeque};

use anyhow::{anyhow, Result};
use bytes::Bytes;

pub struct MailBox<Content>
where
    Content: Clone + From<Bytes> + Into<Bytes>,
{
    mail_list: RefCell<VecDeque<Mail<Content>>>,
}

impl<Content: Clone + From<Bytes> + Into<Bytes>> MailBox<Content> {
    pub fn get_mail(&self) -> Result<Mail<Content>> {
        match self.mail_list.try_borrow_mut()?.pop_front() {
            Some(mail) => Ok(mail),
            None => Err(anyhow!("MailBox is empty")),
        }
    }

    pub fn put_mail(&self, mail: Mail<Content>) -> Result<()> {
        Ok(self.mail_list.try_borrow_mut()?.push_back(mail))
    }
}

pub struct Mail<Content>
where
    Content: Clone + From<Bytes> + Into<Bytes>,
{
    from: String,
    to: String,
    body: Box<Content>,
}

impl<Content: Clone + From<Bytes> + Into<Bytes>> Mail<Content> {
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
            rcs_v.push(rcs.to_owned());
        }

        rcs_v
    }
}

impl<Content> From<(String, String, Bytes)> for Mail<Content>
where
    Content: Clone + From<Bytes> + Into<Bytes>,
{
    fn from(value: (String, String, Bytes)) -> Self {
        Mail::new(value.0, value.1, value.2.into())
    }
}
