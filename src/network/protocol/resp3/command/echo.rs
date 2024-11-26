use crate::network::manager::ConnectionManager;
use crate::network::protocol::resp3::command::Command;

#[derive(Debug)]
pub struct EchoRequest;

impl EchoRequest {
    pub fn new() -> EchoRequest {
        EchoRequest {}
    }
}

impl Command for EchoRequest {
    fn some_fn(&self, connection_manager: &ConnectionManager) {
        connection_manager.write_to_stream("+PONG\r\n");
    }
}