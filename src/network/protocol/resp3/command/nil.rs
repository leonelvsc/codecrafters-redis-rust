use std::io::Write;
use crate::network::manager::ConnectionManager;
use crate::network::protocol::HasPendingRead;
use crate::network::protocol::resp3::command::Command;
use crate::network::protocol::resp3::command::echo::EchoRequest;

#[derive(Debug)]
pub struct NilRequest;

impl NilRequest {
    pub fn new() -> NilRequest {
        NilRequest {}
    }
}

impl HasPendingRead for NilRequest {
    fn has_pending_read(&self) -> bool {
        false
    }
}

impl<W: Write> Command<W> for NilRequest {
    fn process_line(&mut self, line: &String) {
        
    }

    fn process(&self, writer: &mut W) {
        
    }
}