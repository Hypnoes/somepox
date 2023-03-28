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
    pub fn new(file_name: &str) -> Self {
        Self {
            file_name: file_name.to_string(),
        }
    }

    fn get_file_handler(&self) -> Result<File> {
        File::options()
            .create(true)
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
    use std::{fmt::Display, fs, path::Path};

    use anyhow::Result;

    use crate::logbackend::Writable;

    use super::FileLogBackend;

    impl Display for FileLogBackend {
        fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    const TEST_FILE_NAME: &str = "test.log";

    fn prepare_test() -> Result<()> {
        if Path::new(TEST_FILE_NAME).exists() {
            let _ = fs::remove_file(TEST_FILE_NAME).unwrap();
        }

        Ok(())
    }

    #[test]
    fn file_logbackend_write_test() {
        prepare_test().unwrap();

        let backend = FileLogBackend::new(TEST_FILE_NAME);

        backend.write(1, "test-1\n".into()).unwrap();
        backend.write(2, "test-2\n".into()).unwrap();
        backend.write(3, "test-3\n".into()).unwrap();

        let raw_content = fs::read(TEST_FILE_NAME).unwrap();
        let binding = String::from_utf8(raw_content).unwrap();
        let log_content: Vec<&str> = binding.split("\n").collect();

        assert_eq!(log_content[0], "test-1");
        assert_eq!(log_content[1], "test-2");
        assert_eq!(log_content[2], "test-3");

        assert!(true, "Error in write file log.")
    }
}
