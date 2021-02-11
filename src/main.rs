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
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
  let mut world = World::new();

  let mut server = Server::new()?;

  let mut session_token: HashMap<mio::Token, uuid::Uuid> = HashMap::new();

  loop {
    let events = server.pump()?;

    for event in events.iter() {
      match event {
        ServerEvent::Connect(token) => {
          let s = Session::new();
          session_token.insert(*token, s.id());
          world.manage_session(s);
        },
        ServerEvent::Disconnect(token) => {
          if let Some(sid) = session_token.get(token) {
            world.drop_session(*sid);
            session_token.remove(token);
          }
        },
        ServerEvent::Read(token, buf) => {
          if let Some(sid) = session_token.get(token) {
            // XXX extremely dumb parser, for now all input is just a text line
            if let Ok(str) = std::str::from_utf8(buf) {
              if let Some(s) = world.get_session_mut(*sid) {
                for line in str.lines() {
                  s.queue_action(SessionAction::Input(line.trim().to_string()));
                }
              }
            }
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
