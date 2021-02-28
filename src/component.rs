use std::collections::VecDeque;

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
