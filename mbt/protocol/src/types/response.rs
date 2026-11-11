use std::{collections::HashMap, path::PathBuf};
use serde::{ Serialize, Deserialize };

use crate::types::{ Server, ServerConfig, State };

#[derive(Debug, Serialize, Deserialize)]
pub struct ResServer {
  name: String,
  description: Option<String>,
  state: State,
  path: PathBuf,
  config: ServerConfig
}

impl From<&Server> for ResServer {
  fn from(value: &Server) -> Self {
    Self { name: value.name.clone(), description: value.description.clone(), state: value.state.clone(), path: value.path.clone(), config: value.config.clone() }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
  NonItem ,
  Servers {
    data: HashMap<i32, ResServer>,
  },
  ServerInfo {
    data: ResServer,
  },
  Configs {
    data: ServerConfig,
  },
  Message {
    data: String,
  },
  Error {
    code: String,
    message: String
  }
}

impl Response {
  pub fn gen_error(code: &str, message: &str) -> Response {
    Response::Error { code: code.to_string(), message: message.to_string() }
  }
}