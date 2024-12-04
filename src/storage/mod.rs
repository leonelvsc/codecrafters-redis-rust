use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Debug)]
struct SavedData {
    data: String,
    timeout: Option<Duration>,
    created_at: Instant,
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
        self.data.write().await.insert(String::from(key), SavedData { data: String::from(value), timeout, created_at: Instant::now() });
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        if let Some(res) = self.data.read().await.get(key) {
            
            if  res.timeout.is_some() {
                if Instant::now().duration_since(res.created_at) < res.timeout? {
                    Some(res.data.clone())
                } else {
                    None
                }
            } else {
                Some(res.data.clone())
            }
        } else {
            None
        }
    }
}