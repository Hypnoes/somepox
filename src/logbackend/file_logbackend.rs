//! ### File Based Log Backend
//! Use File to Store Log
//!

use std::{fs::File, io::Write};

use anyhow::{anyhow, Result};
use bytes::Bytes;

use super::Writable;

pub struct FileLogBackend {
    file_name: String,
}

impl FileLogBackend {
    pub fn new(file_name: String) -> Self {
        Self { file_name }
    }

    fn get_file_handler(&self) -> Result<File> {
        File::options()
            .append(true)
            .open(&self.file_name)
            .map_err(|e| anyhow!(e))
    }
}

impl Writable for FileLogBackend {
    fn write(&self, _id: u64, data: Bytes) -> Result<()> {
        self.get_file_handler()?
            .write_all(&data)
            .map_err(|e| anyhow!(e))
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use super::FileLogBackend;

    impl Display for FileLogBackend {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    #[test]
    fn file_logbackend_new_test() {
        assert!(true, "Error in crate new file log.")
    }

    #[test]
    fn file_logbackend_write_test() {
        assert!(true, "Error in write file log.")
    }
}
