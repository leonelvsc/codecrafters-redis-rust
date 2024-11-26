use crate::network::protocol::resp3::command::Command;

#[derive(Debug)]
pub struct EchoRequest;

impl EchoRequest {
    pub fn new() -> EchoRequest {
        EchoRequest {}
    }
}

impl Command for EchoRequest {
    fn some_fn(self: Box<Self>) {
        todo!()
    }
}