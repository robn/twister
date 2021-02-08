#[macro_use]
extern crate lazy_static;

mod traits;
mod message;
mod channel;
mod session;
mod world;

use crate::world::world;
use crate::channel::Channel;
use crate::session::Session;

fn main() {
  let c = Channel::new();
  let c_id = c.id();

  let s1 = Session::new();
  let s1_id = s1.id();
  let s2 = Session::new();
  let s2_id = s2.id();

  world().manage_channel(c);

  world().manage_session(s1);
  world().manage_session(s2);

  if let Some(c) = world().get_channel_mut(c_id) {
    c.add_session(s1_id);
    c.add_session(s2_id);
  }
}
