use uuid::Uuid;

use crate::traits::{Receiver, Sender};
use crate::message::Message;

pub struct Session {
  id: Uuid,
}

impl Session {
  pub fn new() -> Self {
    Session {
      id: Uuid::new_v4(),
    }
  }

  pub fn id(&self) -> Uuid {
    self.id
  }
}

impl Receiver for Session {
  fn receive(&self, s: &dyn Sender, msg: &Message) {
  }
}

impl Sender for Session {
}
