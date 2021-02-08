use uuid::Uuid;

use crate::traits::{Receiver, Sender};
use crate::message::Message;

pub struct Channel {
  id: Uuid,
}

impl Channel {
  pub fn new() -> Self {
    Channel {
      id: Uuid::new_v4(),
    }
  }

  pub fn id(&self) -> Uuid {
    self.id
  }
}

impl Receiver for Channel {
  fn receive(&self, s: &dyn Sender, msg: &Message) {
  }
}

impl Sender for Channel {
}
