use std::io::Write;
use crate::network::protocol::HasPendingRead;
use crate::network::protocol::resp3::command::Command;
use crate::network::protocol::resp3::data_types::DataType;

#[derive(Debug)]
pub struct BulkString {
    length: usize,
    data: String
}

impl BulkString {
    pub fn new() -> BulkString {
        BulkString { length: 0, data: "".to_string() }
    }
    
    pub fn set_data(&mut self, data: String) {
        self.data = data;
        self.length = self.data.len();
    }
    
    pub fn len(&self) -> usize {
        self.length
    }
}

impl HasPendingRead for BulkString {
    fn has_pending_read(&self) -> bool {
        self.data.len() == 0
    }
}

impl DataType for BulkString {
    fn process_line(&mut self, line: &str) {
        todo!()
    }

    fn serialize(&self) -> String {
        format!("${}\r\n{}\r\n", self.data.chars().count(), self.data)
    }
}