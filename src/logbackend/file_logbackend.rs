use std::{cell::RefCell, fs::File, io::Write};

use anyhow::{anyhow, Result};
use bytes::Bytes;

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
    fn write(&self, _id: u64, data: Bytes) -> Result<()> {
        match &(self.file) {
            Some(file_handler) => file_handler
                .borrow_mut()
                .write_all(&data)
                .map_err(|e| e.into()),
            None => Err(anyhow!("log file not open.")),
        }
    }
}
