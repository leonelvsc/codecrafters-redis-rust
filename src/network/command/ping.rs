use crate::network::command::Command;
use bytes::Bytes;

#[derive(Debug)]
pub struct PingRequest;

impl Command for PingRequest {

    fn process(&self) -> String {
        String::from("+PONG\r\n")
    }

    fn needs_more_reading(&self) -> bool {
        false
    }

    fn set_data(&mut self, _data: Bytes) {
    }


    fn get_data(&self) -> Bytes {
        Bytes::new()
    }
}