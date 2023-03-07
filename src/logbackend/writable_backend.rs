use bytes::Bytes;

use anyhow::Result;

pub trait Writable {
    fn write(&self, data: Bytes) -> Result<()>;
}
