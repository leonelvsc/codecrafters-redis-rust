use std::io::Write;
use std::rc::Rc;
use crate::network::manager::ConnectionManager;
use crate::network::protocol::resp3::command::Command;

#[derive(Debug)]
pub struct PingRequest;

impl PingRequest {
    pub fn new() -> PingRequest {
        PingRequest {}
    }
}

impl<W: Write> Command<W> for PingRequest {
    fn some_fn(&self, writer: &mut W) {
        writer.write_all(b"+PONG\r\n");
    }
}