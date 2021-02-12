mod message;
//mod channel;
mod session;
mod world;
mod server;

use crate::world::World;
use crate::server::Server;
use crate::session::Session;
use crate::message::{ServerEvent,SessionAction};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let mut world = World::new();

  let mut server = Server::new()?;

  loop {
    let events = server.pump()?;

    for event in events.iter() {
      match event {
        ServerEvent::Connect(sid) => {
          let s = Session::new(*sid);
          world.manage_session(s);
        },
        ServerEvent::Disconnect(sid) => {
          world.drop_session(*sid);
        },
        ServerEvent::Read(sid, str) => {
          if let Some(s) = world.get_session_mut(*sid) {
            s.queue_action(SessionAction::Input(str.to_string()));
          }
        },
      }
    }

    world.process();

    /*
    for &m in messages.iter() {
      if let Some(s) = world.get_session(m.from()) {
        s.receive(&world, m);
      }
    }
    */
  }

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
}
