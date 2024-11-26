use crate::network::protocol::resp3::command::Command;

#[derive(Debug)]
pub struct HeyRequest;

impl HeyRequest {
    pub fn new() -> HeyRequest {
        HeyRequest {}
    }
}

impl Command for HeyRequest {
    fn some_fn(self: Box<Self>) {
        todo!()
    }
}