use std::io::Write;
use crate::network::protocol::HasPendingRead;
use crate::network::protocol::resp3::command::Command;
use crate::network::protocol::resp3::data_types::DataType;

#[derive(Debug)]
pub struct Nil {}

impl Nil {
    pub fn new() -> Nil {
        Nil {}
    }
}

impl HasPendingRead for Nil {

    fn has_pending_read(&self) -> bool {
        false
    }
}

impl DataType for Nil {
    fn process_line(&mut self, line: &str) {
    }
    fn serialize(&self) -> String {
        "".to_string()
    }
}