use crate::message::Message;

pub trait Sender {
}

pub trait Receiver {
  fn receive(&self, s: &dyn Sender, msg: &Message);
}

