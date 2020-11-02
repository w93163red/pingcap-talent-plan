use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;
use std::{collections::HashMap, fs::OpenOptions};
use std::{fs::File, path::PathBuf};

pub struct KvStore {
    map: HashMap<String, String>,
    log_file: File,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            map: HashMap::new(),
            log_file: open_log_file(),
        }
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).cloned())
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::Set {
            key: key.clone(),
            value: value.clone(),
        };
        let log_string: String = serde_json::to_string(&command).unwrap();
        serde_json::to_writer(&self.log_file, &log_string)?;
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

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Rm { key: String },
}

fn open_log_file() -> File {
    let file_name = "/home/ling/pingcap-talent-plan/log.txt";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)
        .expect("file cannot open");
    file
}
