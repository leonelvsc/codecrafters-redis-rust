use std::sync::Arc;
use crate::network::command::Command;
use bytes::Bytes;
use crate::storage::MemoryStorage;

#[derive(Debug)]
pub struct NilRequest;

impl Command for NilRequest {

    fn process(&self) -> String {
        String::new()
    }

    fn needs_more_reading(&self) -> bool {
        false
    }

    fn set_data(&mut self, _data: Bytes) {
    }

    fn get_data(&self) -> Bytes {
        Bytes::new()
    }
}