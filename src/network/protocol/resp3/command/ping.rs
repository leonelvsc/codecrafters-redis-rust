use std::io::Write;
use std::rc::Rc;
use crate::network::manager::ConnectionManager;
use crate::network::protocol::HasPendingRead;
use crate::network::protocol::resp3::command::Command;

#[derive(Debug)]
pub struct PingRequest;

impl PingRequest {
    pub fn new() -> PingRequest {
        PingRequest {}
    }
}

impl HasPendingRead for PingRequest {
    fn has_pending_read(&self) -> bool {
        false
    }
}

impl<W: Write> Command<W> for PingRequest {
    fn process_line(&mut self, _line: &String) {
    }

    fn process(&self, writer: &mut W) {
        writer.write_all(b"+PONG\r\n").expect("TODO: can't write to stream");
    }
}