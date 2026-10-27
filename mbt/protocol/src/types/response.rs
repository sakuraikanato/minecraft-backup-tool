use std::{collections::HashMap, path::PathBuf};
use serde::{ Serialize, Deserialize };

use crate::types::{ ServerConfig, State };

#[derive(Debug, Serialize, Deserialize)]
pub struct ResServer {
  name: String,
  description: Option<String>,
  state: State,
  path: PathBuf
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataOrError<T> {
  Data { data: T },
  Error
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
  NonItem {
    success: bool,
    error: Option<String>
  },
  Servers {
    success: bool,
    data: DataOrError<HashMap<i32, ResServer>>,
  },
  Configs {
    success: bool,
    data: DataOrError<ServerConfig>
  },
  Message {
    success: bool,
    data: DataOrError<String>,
  }
}