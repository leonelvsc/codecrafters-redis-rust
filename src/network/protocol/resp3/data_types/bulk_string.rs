use std::io::Write;
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
}

impl DataType for BulkString {
    fn process_line(&mut self, line: &str) {
        todo!()
    }
}