use crate::network::command::Command;
use crate::storage::MemoryStorage;
use bytes::Bytes;
use std::str::from_utf8;
use std::sync::Arc;
use tokio::runtime::Handle;

#[derive(Debug)]
pub struct GetRequest{
    data: Bytes,
    memory_storage: Arc<MemoryStorage>
}

impl GetRequest {
    pub fn new(db: Arc<MemoryStorage>) -> GetRequest {
        GetRequest { data: Bytes::new(), memory_storage: db }
    }
}

impl Command for GetRequest {

    fn process(&self) -> String {
        if self.data.len() == 0 {
            return String::new();
        }

        let s =  from_utf8(self.data.as_ref()).expect("Error converting data to string");
        
        let result = tokio::task::block_in_place(|| {
            let handle = Handle::current();
            handle.block_on(self.memory_storage.get(s)).unwrap()
        });
        
        format!("+{}\r\n", result)
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