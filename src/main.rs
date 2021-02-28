//mod channel;
//mod session;
//mod world;
mod server;
mod component;
mod lobby;
mod command;

//use crate::world::World;
use crate::server::Server;

use std::error::Error;

use hecs::*;

fn main() -> Result<(), Box<dyn Error>> {
  let mut world = World::new();

  let mut server = Server::new()?;

  loop {
    server.update(&mut world)?;

    lobby::update(&mut world);

    command::update(&mut world);
  }
}

    /*
    for &m in messages.iter() {
      if let Some(s) = world.get_session(m.from()) {
        s.receive(&world, m);
      }
    }
    */

/*
  let c = Channel::new();
  let c_id = c.id();

  let s1 = Session::new();
  let s1_id = s1.id();
  let s2 = Session::new();
  let s2_id = s2.id();

  world.manage_channel(c);

  world.manage_session(s1);
  world.manage_session(s2);

  if let Some(c) = world.get_channel_mut(c_id) {
    c.add_session(s1_id);
    c.add_session(s2_id);
  }

  let m = Message::Text("hello".to_string());

  if let Some(c) = world.get_channel(c_id) {
    c.receive(&world, s1_id, &m);
  }
*/
