use uuid::Uuid;

use crate::traits::{MessageReceiver, Sender};
use crate::world::World;
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

impl MessageReceiver for Session {
  fn queue(&mut self, msg: Message) {
    println!("session {} queued message: {:?}", self.id, msg);
  }
}

impl Sender for Session {
}
