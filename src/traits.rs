use uuid::Uuid;

use crate::world::World;
use crate::message::Message;

pub trait Sender {
}

pub trait Receiver {
  fn receive(&self, world: &World, sender_id: Uuid, msg: &Message);
}

