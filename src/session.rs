use uuid::Uuid;

use crate::traits::{MessageReceiver, Sender};
use crate::world::World;
use crate::message::Message;

#[derive(Default)]
pub struct Session {
  id:    Uuid,
  queue: Vec<Message>,
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
    self.queue.push(msg);
  }

  fn pump(&mut self) {
    println!("session {} pump", self.id);

    for msg in self.queue.iter() {
      println!("session {} pump message: {:?}", self.id, msg);

      match msg {
        Message::Input(text) => {
          println!("session {} input: {}", self.id, text);
        },
        _ => unimplemented!(),
      }
    }

    self.queue.truncate(0);
  }
}

impl Sender for Session {
}
