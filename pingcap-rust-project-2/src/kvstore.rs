use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>
}


impl KvStore {
    pub fn new() -> Self {
        KvStore {
            map: HashMap::new()
        }
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).cloned())
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.map.insert(key, value);
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.map.remove(&key);
        Ok(())
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        unimplemented!()
    }
}

