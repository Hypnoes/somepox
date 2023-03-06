use bytes::Bytes;

use crate::error::GeneralError;

pub trait Writable {
    fn write(&self, data: Bytes) -> Result<(), GeneralError>;
}
