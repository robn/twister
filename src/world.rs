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
  pub fn get_session_mut(&mut self, id: Uuid) -> Option<&mut Session> {
    self.sessions.get_mut(&id)
  }
}

use std::sync::{Mutex,MutexGuard};

lazy_static!(
  static ref _WORLD: Mutex<World> = {
    Mutex::new(World::new())
  };
);

pub fn world() -> MutexGuard<'static, World> {
  _WORLD.lock().unwrap()
}
