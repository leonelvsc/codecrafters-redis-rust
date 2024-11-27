use std::io::Write;
use crate::network::manager::ConnectionManager;
use crate::network::protocol::resp3::command::Command;

#[derive(Debug)]
pub struct EchoRequest;

impl EchoRequest {
    pub fn new() -> EchoRequest {
        EchoRequest {}
    }
}

impl<W: Write> Command<W> for EchoRequest {
    fn some_fn(&self, writer: &mut W) {
        writer.write_all(b"+PONG\r\n");
    }
}