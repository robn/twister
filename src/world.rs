use uuid::Uuid;
use std::collections::HashMap;

use crate::session::Session;
use crate::channel::Channel;
use crate::traits::MessageReceiver;

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
  pub fn get_channel(&self, id: Uuid) -> Option<&Channel> {
    self.channels.get(&id)
  }
  pub fn get_channel_mut(&mut self, id: Uuid) -> Option<&mut Channel> {
    self.channels.get_mut(&id)
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
  pub fn get_session(&self, id: Uuid) -> Option<&Session> {
    self.sessions.get(&id)
  }
  pub fn get_session_mut(&mut self, id: Uuid) -> Option<&mut Session> {
    self.sessions.get_mut(&id)
  }


  pub fn pump(&mut self) {
    for (sid, mut s) in self.sessions.iter_mut() {
      s.pump();
    }
  }
}
