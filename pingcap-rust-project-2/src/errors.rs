use thiserror::Error;
use std::io;
#[derive(Error, Debug)]
pub enum KvsError {
    #[error("cannot get the key")]
    KeyNotFound(String),
    #[error("cannot open the file")]
    FileNotFound(#[from] io::Error),
}