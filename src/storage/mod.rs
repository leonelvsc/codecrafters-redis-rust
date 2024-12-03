use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct MemoryStorage {
    data: RwLock<HashMap<String, String>>
}

impl MemoryStorage {
    pub fn new() -> MemoryStorage {
        MemoryStorage {
            data: RwLock::new(HashMap::new()),
        }
    }

    pub async fn set(&self, key: &str, value: &str) {
        self.data.write().await.insert(String::from(key), String::from(value));
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        self.data.read().await.get(key).cloned()
    }
}