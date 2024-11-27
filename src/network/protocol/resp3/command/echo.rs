use std::io::Write;
use crate::network::manager::ConnectionManager;
use crate::network::protocol::HasPendingRead;
use crate::network::protocol::resp3::command::Command;
use crate::network::protocol::resp3::data_types::bulk_string::BulkString;
use crate::network::protocol::resp3::data_types::DataType;

#[derive(Debug)]
pub struct EchoRequest {
    data: BulkString,
}

impl EchoRequest {
    pub fn new() -> EchoRequest {
        EchoRequest {
            data: BulkString::new(),
        }
    }
}

impl HasPendingRead for EchoRequest {
    fn has_pending_read(&self) -> bool {
        self.data.len() == 0
    }
}

impl<W: Write> Command<W> for EchoRequest {
    fn process_line(&mut self, line: &String) {
        // todo: refactor data types
        if line.chars().all(char::is_alphanumeric) {
            self.data.set_data(line.clone());
        }
    }

    fn process(&self, writer: &mut W) {
        if self.data.len() == 0 {
            return;
        }

        writer.write_all(self.data.serialize().as_bytes()).expect("TODO: can't write to stream");
    }
}