use uuid::Uuid;

use crate::message::Message;

pub trait Sender {
}

pub trait Receiver {
  fn receive(&self, sender_id: Uuid, msg: &Message);
}

