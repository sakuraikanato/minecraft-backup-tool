use std::{collections::HashMap, path::PathBuf};
use serde::{ Serialize, Deserialize };

use crate::types::{ State };

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
  Error { error: T}
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
  Message {
    success: bool,
    data: DataOrError<String>,
  }
}