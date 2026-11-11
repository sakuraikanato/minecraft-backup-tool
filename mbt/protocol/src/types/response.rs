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

#[derive(Debug, Serialize, Deserialize)]
pub struct ResError {
  code: String,
  message: String
}

impl From<&Server> for ResServer {
  fn from(value: &Server) -> Self {
    Self { name: value.name.clone(), description: value.description.clone(), state: value.state.clone(), path: value.path.clone() }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
  NonItem {
    success: bool,
    error: Option<ResError>
  },
  Servers {
    success: bool,
    data: Option<HashMap<i32, ResServer>>,
    error: Option<ResError>
  },
  Configs {
    success: bool,
    data: Option<ServerConfig>,
    error: Option<ResError>
  },
  Message {
    success: bool,
    data: Option<String>,
    error: Option<ResError>
  }
}