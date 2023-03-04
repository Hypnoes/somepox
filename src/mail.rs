use std::{
    cell::{BorrowMutError, RefCell},
    collections::VecDeque,
};

use crate::error::GeneralError;

pub struct MailBox<Content>
where
    Content: Clone + From<Vec<u8>> + Into<Vec<u8>>,
{
    mail_list: RefCell<VecDeque<Mail<Content>>>,
}

impl<Content: Clone + From<Vec<u8>> + Into<Vec<u8>>> MailBox<Content> {
    pub fn get_mail(&self) -> Result<Mail<Content>, GeneralError> {
        match self.mail_list.try_borrow_mut()?.pop_front() {
            Some(mail) => Ok(mail),
            None => Err("MailBox is empty".into()),
        }
    }

    pub fn put_mail(&self, mail: Mail<Content>) -> Result<(), GeneralError> {
        Ok(self.mail_list.try_borrow_mut()?.push_back(mail))
    }
}

pub struct Mail<Content>
where
    Content: Clone + From<Vec<u8>> + Into<Vec<u8>>,
{
    from: String,
    to: String,
    content: Box<Content>,
}

impl<Content: Clone + From<Vec<u8>> + Into<Vec<u8>>> Mail<Content> {
    pub fn new(from: String, to: String, content: Content) -> Mail<Content> {
        Mail {
            from: from,
            to: to,
            content: Box::new(content),
        }
    }

    pub fn content(&self) -> Content {
        (*self.content).clone()
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

impl<Content> Into<Mail<Content>> for (String, String, Vec<u8>)
where
    Content: Clone + From<Vec<u8>> + Into<Vec<u8>>,
{
    fn into(self) -> Mail<Content> {
        Mail::new(self.0, self.1, self.2.into())
    }
}

impl From<BorrowMutError> for GeneralError {
    fn from(value: BorrowMutError) -> Self {
        format!("{:?}", value).into()
    }
}
