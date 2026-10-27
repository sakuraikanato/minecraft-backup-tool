use std::collections::HashMap;

use protocol::types::{self, Server, ServerConfigUpdate, ServerUpdate, State, StdIo};

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

  pub fn change_io(&mut self, id: i32, io: StdIo) -> types::Result<()> {
    let res = self.manager.update(id, ServerUpdate {
      io: Some(io), ..Default::default() 
    });
    match res {
      Ok(_) => Ok(()),
      Err(e) => Err(e)
    }
  }

  pub fn change_config(&mut self, id: i32, config: ServerConfigUpdate) -> types::Result<()> {
    let res = self.manager.update(id, ServerUpdate { 
      config: Some(config), ..Default::default() 
    });
    match res {
      Ok(_) => Ok(()),
      Err(e) => Err(e)
    }
  }
}
