#![allow(unused)]

mod file_logbackend;
mod heap_logbackend;

pub use file_logbackend::FileLogBackend;
pub use heap_logbackend::HeapLogBackend;

use anyhow::Result;
use bytes::Bytes;

pub trait Writable {
    fn write(&self, id: u64, data: Bytes) -> Result<()>;
}

pub trait Queryable {
    fn query(&self, id: u64) -> Result<Bytes>;
}

pub trait LogBackend: Queryable + Writable {}
