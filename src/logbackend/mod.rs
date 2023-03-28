mod file_logbackend;
mod heap_logbackend;
mod writable_backend;

pub use file_logbackend::FileLogBackend;
pub use heap_logbackend::HeapLogBackend;
pub use writable_backend::Writable;

pub fn file_logbackend() -> FileLogBackend {
    todo!()
}
pub fn heap_logbackend() -> HeapLogBackend {
    todo!()
}
