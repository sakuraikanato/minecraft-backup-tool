use serde::{ Serialize, Deserialize };

use std::io;
use serde_json;

#[derive(Debug)]
pub enum MmsError {
  Io(io::Error),
  Json(serde_json::Error),
  NotFound,
}