mod file_logbackend;
mod heap_logbackend;

pub use file_logbackend::FileLogBackend;
pub use heap_logbackend::HeapLogBackend;

use anyhow::Result;
use bytes::Bytes;

pub trait Writable {
    fn write(&self, id: u64, data: Bytes) -> Result<()>;
}
