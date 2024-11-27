use std::io::Write;
use std::rc::Rc;
use crate::network::manager::ConnectionManager;
use crate::network::protocol::resp3::command::Command;

#[derive(Debug)]
pub struct HeyRequest;

impl HeyRequest {
    pub fn new() -> HeyRequest {
        HeyRequest {}
    }
}

impl<W: Write> Command<W> for HeyRequest {
    fn some_fn(&self, writer: &mut W) {
        writer.write_all(b"+PONG\r\n");
    }
}