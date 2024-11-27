use crate::network::protocol::resp3::command::Command;
use crate::network::protocol::resp3::data_types::DataType;
use std::io::Write;
use crate::network::protocol::HasPendingRead;

#[derive(Debug)]
pub struct Array {
    length: usize,
    data: Vec<Box<dyn DataType>>,
}

impl Array {
    pub fn new() -> Array {
        Array {
            length: 0,
            data: Vec::new(),
        }
    }
}

impl HasPendingRead for Array {
    fn has_pending_read(&self) -> bool {
        self.data.len() == 0
    }
}

impl DataType for Array {
    fn process_line(&mut self, line: &str) {
        println!("line before parse: {}", line);

        if self.length == 0 {
            match line.parse::<usize>() {
                Ok(length) => {
                    self.length = length;
                    println!("Array length: {}", self.length);
                }
                Err(_) => {
                    panic!("Fail to parse array length")
                }
            }
        } else {
            
        }
    }
    fn serialize(&self) -> String {
        "to-do".to_string()
    }
}
