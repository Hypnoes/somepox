use bytes::Bytes;

use anyhow::Result;

pub trait Writable {
    fn write(&self, id: u64, data: Bytes) -> Result<()>;
}
