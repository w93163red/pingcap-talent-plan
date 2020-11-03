use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io;
use std::{collections::HashMap, fs::OpenOptions};
use std::{env, io::BufRead, io::BufReader};
use std::{fs::File, path::PathBuf};
use std::hint::unreachable_unchecked;
use std::io::Write;

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
        // read all logs from the file
        let reader = BufReader::new(&self.log_file);
        for line in reader.lines() {
            let data = serde_json::from_str::<Command>(&line.unwrap())?;
            match data {
                Command::Set {key: k, value: v} => self.map.insert(k, v),
                Command::Rm {key: k} => self.map.remove(&k),
                _ => {unimplemented!()}
            };
        }
        // let data: Command = serde_json::from_reader(reader)?;
        Ok(self.map.get(&key).cloned())
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::Set {
            key: key.clone(),
            value: value.clone(),
        };
        serde_json::to_writer(&self.log_file, &command)?;
        self.log_file.write("\n".as_bytes());
        self.map.insert(key, value);
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let command = Command::Rm { key: key.clone() };
        serde_json::to_writer(&self.log_file, &command)?;
        self.map.remove(&key);
        Ok(())
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        // since the test is only parsing the directory
        let mut path: PathBuf = path.into();
        path.push("log.txt");
        Ok(KvStore {
            map: HashMap::new(),
            path: path.clone(),
            log_file: open_log_file(path),
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Rm { key: String },
}

fn open_log_file(path: impl Into<PathBuf>) -> File {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(path.into())
        .expect("file cannot open");
    file
}
