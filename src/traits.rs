use uuid::Uuid;

use crate::world::World;
use crate::message::Message;

pub trait Sender {
}

pub trait MessageReceiver {
  fn queue(&mut self, msg: Message);
  fn pump(&mut self);
}

