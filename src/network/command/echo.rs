use crate::network::command::Command;
use std::io::Write;
use bytes::Bytes;

pub struct EchoRequest {
    data: Bytes,
}

impl Command for EchoRequest {

    fn process(&self) -> &[u8] {
        if self.data.len() == 0 {
            return &[];
        }

        self.data.as_ref()
    }
}