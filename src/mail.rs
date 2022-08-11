use std::{borrow::Borrow, collections::VecDeque, error::Error};

use tokio::net::UdpSocket;

pub struct MailBox<T> {
    mail_list: VecDeque<Mail<T>>,
    port: UdpSocket,
    address_book: Vec<(String, String)>,
}

impl<T> MailBox<T> {
    pub fn get(&mut self) -> Option<Mail<T>> {
        self.mail_list.pop_front()
    }

    pub fn put(&mut self, mail: Mail<T>) -> Result<(), Box<dyn Error>> {
        Ok(self.mail_list.push_back(mail))
    }

    pub fn address_book(&self) -> &Vec<(String, String)> {
        self.address_book.borrow()
    }
}

pub struct Mail<T> {
    from: String,
    to: String,
    content: T,
}

impl<T> Mail<T> {
    pub fn content(&self) -> &T {
        &self.content
    }
}
