use std::{cell::RefCell, collections::VecDeque, error::Error, rc::Rc};

pub struct MailBox<T> {
    mail_list: RefCell<VecDeque<Mail<T>>>,
    address_book: Box<Vec<(String, String)>>,
}

impl<T> MailBox<T> {
    pub fn get(&self) -> Option<Mail<T>> {
        self.mail_list.borrow_mut().pop_front()
    }

    pub fn put(&self, mail: Mail<T>) -> Result<(), Box<dyn Error>> {
        Ok(self.mail_list.borrow_mut().push_back(mail))
    }

    pub fn address_book(&self) -> &Vec<(String, String)> {
        &self.address_book
    }
}

pub struct Mail<T> {
    from: String,
    to: String,
    content: Box<T>,
}

impl<T> Mail<T> {
    pub fn content(&self) -> &T {
        &self.content
    }
}

impl<T: Clone> Mail<T> {
    pub fn new(from: &str, to: &str, content: T) -> Mail<T> {
        Mail {
            from: from.to_string(),
            to: to.to_string(),
            content: Box::new(content.to_owned()),
        }
    }
}
