use std::collections::HashMap;

use protocol::types::Server;

use crate::managers::Manager;

pub struct ManagerExt {
  manager: Manager
}

impl ManagerExt {
  pub fn new() -> Self {
    Self { manager: Manager { servers: HashMap::<i32, Server>::new() } }
  }
}