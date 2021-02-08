mod traits;
mod message;
mod channel;
mod session;
mod world;

use crate::world::World;
use crate::channel::Channel;
use crate::session::Session;

fn main() {
  let mut world = World::new();

  let c = Channel::new();
  let s = Session::new();

  world.manage_channel(c);
  world.manage_session(s);
}
