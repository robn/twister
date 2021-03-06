use std::collections::{VecDeque,HashSet};

use hecs::*;

#[derive(Debug)]
pub struct LineIO {
  pub input:  VecDeque<String>,
  pub output: VecDeque<String>,
}

#[derive(Debug)]
pub enum Lobby {
  Start,
  Username,
  End(String),
}

#[derive(Debug)]
pub struct Name(pub String);


#[derive(Debug)]
pub struct Channel {
  pub members:  HashSet<Entity>,
}

#[derive(Debug)]
pub enum ChannelEvent {
  Join(Entity, Entity),
}
