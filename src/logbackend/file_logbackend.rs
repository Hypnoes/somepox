use std::{cell::RefCell, fs::File, io::Write};

use bytes::Bytes;

use crate::error::GeneralError;

use super::Writable;

pub struct FileLogBackend {
    file_name: String,
    file: Option<RefCell<File>>,
}

impl FileLogBackend {
    pub fn new(file_name: String) -> Self {
        Self {
            file_name,
            file: None,
        }
    }

    pub fn open(&mut self) {
        self.file = File::options()
            .append(true)
            .open(&(self.file_name))
            .map(|f| RefCell::new(f))
            .ok();
    }
}

impl Writable for FileLogBackend {
    fn write(&self, data: Bytes) -> Result<(), GeneralError> {
        match &(self.file) {
            Some(file_handler) => file_handler
                .borrow_mut()
                .write_all(&data)
                .map_err(|e| e.into()),
            None => Err("log file not open.".into()),
        }
    }
}
