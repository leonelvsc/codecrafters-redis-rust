use crate::network::command::Command;
use bytes::Bytes;

#[derive(Debug)]
pub struct DataWrapper {
    data: Bytes,
}

impl DataWrapper {
    pub fn new(data: Bytes) -> DataWrapper {
        DataWrapper { data }
    }
}

impl Command for DataWrapper {

    fn process(&self) -> String {
        String::new()
    }

    fn needs_more_reading(&self) -> bool {
        false
    }

    fn set_data(&mut self, data: Bytes) {
        self.data = data;
    }
    
    fn get_data(&self) -> Bytes {
        self.data.clone()
    }
}