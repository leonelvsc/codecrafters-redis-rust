use std::io::Write;
use crate::network::protocol::resp3::command::Command;
use crate::network::protocol::resp3::data_types::DataType;

#[derive(Debug)]
pub struct Nil {}

impl Nil {
    pub fn new() -> Nil {
        Nil {}
    }
}

impl DataType for Nil {
    fn process_line(&mut self, line: &str) {
    }
}