use uuid::Uuid;

use crate::traits::{Receiver, Sender};
use crate::message::Message;

#[derive(Default)]
pub struct Session {
  id: Uuid,
}

impl Session {
  pub fn new() -> Self {
    Session {
      id: Uuid::new_v4(),
      ..Default::default()
    }
  }

  pub fn id(&self) -> Uuid {
    self.id
  }
}

impl Receiver for Session {
  fn receive(&self, sender_id: Uuid, msg: &Message) {
    println!("session {} received message: {:?}", self.id, msg);
  }
}

impl Sender for Session {
}
