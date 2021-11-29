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

  // I think the main loop is like:
  //
  // - run each system's update(). each run gets an immutable handle to the global data (world,
  //   catalog, etc).  it does lookups and whatever else to decide what to do, and returns a list of
  //   "actions", which are just enum values carrying the modification to be made
  //
  // - run each system's apply(), which gets a mutable handle to the global data, and the action
  //   list. it can make appropriate changes to the world for the given action, if it cares
  //
  // the idea is, sometimes multiple systems will need to respond to an action (eg "user quit", which
  // might see a channel/group remove that user and the server do a disconnect. but calling a
  // "user_quit()" method from inside update means we're likely to already have a reference out for
  // the world at the time we need to modify it, which isn't going to work and necessitates a
  // two-step process anyway within the system, but then would also mean that the action function
  // would have to know about multiple systems anyway. so "collect, then apply" seems to be a
  // simple generalisation of the idea.

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
