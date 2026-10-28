use std::{collections::HashMap, path::PathBuf};
use serde::{ Serialize, Deserialize };

use crate::types::{ Server, ServerConfig, State };

#[derive(Debug, Serialize, Deserialize)]
pub struct ResServer {
  name: String,
  description: Option<String>,
  state: State,
  path: PathBuf
}

impl From<&Server> for ResServer {
  fn from(value: &Server) -> Self {
      Self { name: value.name.clone(), description: value.description.clone(), state: value.state.clone(), path: value.path.clone() }
  }
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