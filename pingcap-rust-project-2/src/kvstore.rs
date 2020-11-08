use anyhow::bail;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use std::{collections::HashMap, fs::OpenOptions};
use std::{fs::File, path::PathBuf};
use std::{io::BufRead, io::BufReader};

pub struct KvStore {
    map: HashMap<String, String>,
    path: PathBuf,
    log_file: File,
}

impl KvStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        KvStore::open(path).expect("cannot open the file")
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).cloned())
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::Set {
            key: key.clone(),
            value: value.clone(),
        };
        serde_json::to_writer(&self.log_file, &command)?;
        self.log_file.write(b"\n");
        self.map.insert(key, value);
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let command = Command::Rm { key: key.clone() };
        serde_json::to_writer(&self.log_file, &command)?;
        if self.map.remove(&key).is_some() {
            Ok(())
        } else {
            bail!("Key not found")
        }
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        // since the test is only parsing the directory
        let mut path: PathBuf = path.into();
        path.push("log.txt");
        let mut kvStore = KvStore {
            map: HashMap::new(),
            path: path.clone(),
            log_file: open_log_file(path.clone()),
        };
        let reader = BufReader::new(kvStore.log_file.try_clone()?);
        for line in reader.lines() {
            let data = serde_json::from_str::<Command>(&line?)?;
            match data {
                Command::Set { key: k, value: v } => kvStore.set(k, v),
                Command::Rm { key: k } => kvStore.remove(k),
                _ => unimplemented!(),
            };
        }
        Ok(kvStore)
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Rm { key: String },
}

fn open_log_file(path: impl Into<PathBuf>) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(path.into())
        .expect("file cannot open")
}
