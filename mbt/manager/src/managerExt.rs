use std::collections::HashMap;

use protocol::types::{self, Server, ServerUpdate, State};

use crate::managers::Manager;

pub struct ManagerExt {
  manager: Manager
}

impl ManagerExt {
  pub fn new() -> Self {
    Self { manager: Manager { servers: HashMap::<i32, Server>::new() } }
  }

  pub fn change_state(&mut self, id: i32, state: State) -> types::Result<()> {
    let res = self.manager.update(id, ServerUpdate {
      state: Some(state),
      ..Default::default()
    });
    match res {
      Ok(_) => Ok(()),
      Err(e) => Err(e)
    }
  }
}
