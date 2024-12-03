use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::RwLock;

#[derive(Debug)]
struct SavedData {
    data: String,
    timeout: Option<Duration>,
}

#[derive(Debug)]
pub struct MemoryStorage {
    data: RwLock<HashMap<String, SavedData>>
}

impl MemoryStorage {
    pub fn new() -> MemoryStorage {
        MemoryStorage {
            data: RwLock::new(HashMap::new()),
        }
    }

    pub async fn set(&self, key: &str, value: &str, timeout: Option<Duration>) {
        self.data.write().await.insert(String::from(key), SavedData { data: String::from(value), timeout });
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        Some(self.data.read().await.get(key)?.data.clone())
    }
}