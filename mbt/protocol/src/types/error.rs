use serde::{ Serialize, Deserialize };

use std::io;
use serde_json;

#[derive(Debug)]
pub enum Error {
  Io(io::Error),
  Json(serde_json::Error)
}