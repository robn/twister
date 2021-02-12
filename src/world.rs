use uuid::Uuid;
use std::collections::HashMap;

use crate::session::Session;
use crate::message::{WorldAction, ServerAction};

#[derive(Default)]
pub struct World {
  sessions: HashMap<Uuid,Session>,
}

impl World {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn manage_session(&mut self, s: Session) {
    let id = s.id();
    self.sessions.insert(id, s);
    println!("managing session: {}", id);
  }
  pub fn drop_session(&mut self, id: Uuid) {
    self.sessions.remove(&id);
    println!("dropped session: {}", id);
  }
  /*
  pub fn get_session(&self, id: Uuid) -> Option<&Session> {
    self.sessions.get(&id)
  }
  */
  pub fn get_session_mut(&mut self, id: Uuid) -> Option<&mut Session> {
    self.sessions.get_mut(&id)
  }


  pub fn process(&mut self) -> Vec<ServerAction> {
    let mut world_actions = vec!();

    for (_, s) in self.sessions.iter_mut() {
      world_actions.append(&mut s.process_actions());
    }

    let mut server_actions = vec!();

    for action in world_actions.iter() {
      match action {
        WorldAction::Wall(text) => {
          println!("wall: {}", text);
          for (id, _) in self.sessions.iter() {
            server_actions.push(ServerAction::Write(*id, text.to_string()));
          }
        },
      }
    }

    server_actions
  }
}
