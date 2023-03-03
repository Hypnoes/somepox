use std::{
    cell::{BorrowMutError, RefCell},
    collections::VecDeque,
};

use crate::error::GeneralError;

pub struct MailBox<Content> {
    mail_list: RefCell<VecDeque<Mail<Content>>>,
    address_book: Box<Vec<(String, String)>>,
}

impl<Content> MailBox<Content> {
    pub fn get_new_mail(&self) -> Result<Mail<Content>, GeneralError> {
        match self.mail_list.try_borrow_mut()?.pop_front() {
            Some(mail) => Ok(mail),
            None => Err("MailBox is empty".into()),
        }
    }

    pub fn put_mail(&self, mail: Mail<Content>) -> Result<(), GeneralError> {
        Ok(self.mail_list.try_borrow_mut()?.push_back(mail))
    }

    pub fn address_book(&self) -> &Vec<(String, String)> {
        &self.address_book
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
    pub fn content(&self) -> &Content {
        &self.content
    }

    pub fn new(from: &str, to: &str, content: &Content) -> Mail<Content> {
        Mail {
            from: String::from(from),
            to: String::from(to),
            content: Box::new(content.to_owned()),
        }
    }
}

impl From<BorrowMutError> for GeneralError {
    fn from(value: BorrowMutError) -> Self {
        format!("{:?}", value).into()
    }
}
