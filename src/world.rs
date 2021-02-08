use uuid::Uuid;
use std::collections::HashMap;

use crate::session::Session;
use crate::channel::Channel;

#[derive(Default)]
pub struct World {
  channels: HashMap<Uuid,Channel>,
  sessions: HashMap<Uuid,Session>,
}

impl World {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn manage_channel(&mut self, c: Channel) {
    let id = c.id();
    self.channels.insert(id, c);
    println!("managing channel: {}", id);
  }

  pub fn manage_session(&mut self, s: Session) {
    let id = s.id();
    self.sessions.insert(id, s);
    println!("managing session: {}", id);
  }

  /*
  fn get_session(&self, id: Uuid) -> Option<&Session> {
    self.sessions.get(&id)
  }
  */
}
