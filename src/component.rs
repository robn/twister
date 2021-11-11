use std::collections::{VecDeque,HashSet};

use hecs::*;

// anything with a LineIO can receive and send text lines, so a connection of some sort
#[derive(Debug)]
pub struct LineIO {
  pub input:  VecDeque<String>,
  pub output: VecDeque<String>,
}

// connection states. a connection moves through the states, top to bottom
#[derive(Debug)]
pub enum Connection {
  Connected,
  Welcome,
  Username,
  Online,
  Offline,
  Disconnected,
}

#[derive(Debug)]
pub struct Name(pub String);

#[derive(Debug)]
pub struct Channel {
  pub members: HashSet<Entity>,
}

#[derive(Debug)]
pub enum ChannelEvent {
  Join(Entity, Entity),
}
