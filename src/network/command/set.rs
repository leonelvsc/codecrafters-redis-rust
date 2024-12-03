use crate::network::command::Command;
use crate::storage::MemoryStorage;
use bytes::Bytes;
use std::str::from_utf8;
use std::sync::Arc;
use tokio::runtime::Handle;

#[derive(Debug)]
pub struct SetRequest{
    data: (String, String, Option<String>, Option<String>),
    memory_storage: Arc<MemoryStorage>
}

impl SetRequest {
    pub fn new(db: Arc<MemoryStorage>) -> SetRequest {
        SetRequest { data: (String::new(), String::new(), None, None), memory_storage: db }
    }
}

impl Command for SetRequest {

    fn process(&self) -> String {
        tokio::task::block_in_place(|| {
            let handle = Handle::current();
            handle.block_on(self.memory_storage.set(&self.data.0, &self.data.1, None));
        });
        
        "+OK\r\n".to_string()
    }

    fn needs_more_reading(&self) -> bool {
        self.data.0.is_empty() || self.data.1.is_empty()
    }

    fn set_data(&mut self, data: Bytes) {
        let string = from_utf8(data.as_ref()).expect("Error converting data to string");
        
        if self.data.0.is_empty() {
            self.data.0.push_str(string);
        } else if self.data.1.is_empty() {
            self.data.1.push_str(string);
        } else if self.data.2.is_none() {
            self.data.2 = Some(string.to_string());
        } else { 
            self.data.3 = Some(string.to_string());
        }
    }

    fn get_data(&self) -> Bytes {
        Bytes::new()
    }
}