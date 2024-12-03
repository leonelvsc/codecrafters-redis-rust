use crate::network::command::Command;
use bytes::Bytes;
use std::str::from_utf8;

#[derive(Debug)]
pub struct EchoRequest {
    data: Bytes,
}

impl EchoRequest {
    pub fn new() -> EchoRequest {
        EchoRequest { data: Bytes::new() }
    }
}

impl Command for EchoRequest {

    fn process(&self) -> String {
        let s =  from_utf8(self.data.as_ref()).expect("Error converting data to string");
        format!("${}\r\n{}\r\n", s.len(), s)
    }

    fn needs_more_reading(&self) -> bool {
        self.data.len() == 0
    }

    fn set_data(&mut self, data: Bytes) {
        self.data = data;
    }

    fn get_data(&self) -> Bytes {
        self.data.clone()
    }
}