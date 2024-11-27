use std::io::Write;
use crate::network::protocol::HasPendingRead;
use crate::network::protocol::resp3::command::Command;
use crate::network::protocol::resp3::data_types::DataType;

#[derive(Debug)]
pub struct SimpleString {
    length: usize,
    data: String
}

impl SimpleString {
    pub fn new() -> SimpleString {
        SimpleString { length: 0, data: "".to_string() }
    }
}

impl HasPendingRead for SimpleString {
    fn has_pending_read(&self) -> bool {
        self.data.len() == 0
    }
}

impl DataType for SimpleString {
    fn process_line(&mut self, line: &str) {
        todo!()
    }

    fn serialize(&self) -> String {
        format!("+{}\r\n", self.data)
    }
}