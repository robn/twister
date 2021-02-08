#[macro_use]
extern crate lazy_static;

mod traits;
mod message;
mod channel;
mod session;
mod world;

use crate::world::World;
use crate::channel::Channel;
use crate::session::Session;

use std::sync::Mutex;

lazy_static!(
  static ref WORLD: Mutex<World> = {
    Mutex::new(World::new())
  };
);

fn main() {
  let c = Channel::new();
  let c_id = c.id();

  let s1 = Session::new();
  let s1_id = s1.id();
  let s2 = Session::new();
  let s2_id = s2.id();

  WORLD.lock().unwrap().manage_channel(c);

  WORLD.lock().unwrap().manage_session(s1);
  WORLD.lock().unwrap().manage_session(s2);

  if let Some(c) = WORLD.lock().unwrap().get_channel_mut(c_id) {
    c.add_session(s1_id);
    c.add_session(s2_id);
  }
}
