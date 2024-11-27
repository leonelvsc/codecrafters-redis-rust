use std::io::Write;
use crate::network::manager::ConnectionManager;
use crate::network::protocol::resp3::command::Command;

#[derive(Debug)]
pub struct NilRequest;

impl NilRequest {
    pub fn new() -> NilRequest {
        NilRequest {}
    }
}

impl<W: Write> Command<W> for NilRequest {
    fn some_fn(&self, writer: &mut W) {
        
    }
}