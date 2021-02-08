use uuid::Uuid;
use std::collections::HashSet;

use crate::traits::{Receiver, Sender};
use crate::message::Message;

#[derive(Default)]
pub struct Channel {
  id: Uuid,
  sessions: HashSet<Uuid>,
}

impl Channel {
  pub fn new() -> Self {
    Channel {
      id: Uuid::new_v4(),
      ..Default::default()
    }
  }

  pub fn id(&self) -> Uuid {
    self.id
  }

  pub fn add_session(&mut self, id: Uuid) {
    self.sessions.insert(id);
    println!("channel {} adding session: {}", self.id, id);
  }

  pub fn remove_session(&mut self, id: Uuid) {
    self.sessions.remove(&id);
    println!("channel {} removing session: {}", self.id, id);
  }
}

impl Receiver for Channel {
  fn receive(&self, sender_id: Uuid, msg: &Message) {
    println!("channel {} received message: {:?}", self.id, msg);
  }
}

impl Sender for Channel {
}
