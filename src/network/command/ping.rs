use crate::network::command::Command;
use std::io::Write;

#[derive(Debug)]
pub struct PingRequest;

impl Command for PingRequest {

    fn process(&self) -> &[u8] {
        b"+PONG"
    }
}