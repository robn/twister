mod server;
mod component;
mod action;
mod connection;
mod command;
mod channel;
mod global;

use crate::global::Global;
use crate::server::Server;
use crate::action::Action;

use std::error::Error;
use std::collections::{HashSet, HashMap};

use hecs::*;
use crate::component::{Name,Channel};

fn main() -> Result<(), Box<dyn Error>> {
  let mut g = Global {
    world:   World::new(),
    catalog: HashMap::new(),
  };

  g.world.spawn((
    Name("wall".to_string()),
    Channel { members: HashSet::new() },
  ));

  let mut server = Server::new()?;

  loop {
    server.update(&mut g)?;

    let mut actions: Vec<Action> = vec!();

    actions.append(&mut connection::update(&mut g));

    actions.append(&mut command::update(&mut g));

    //channel::update(&mut g);

    action::apply(&mut g, actions);
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
