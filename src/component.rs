#[derive(Debug)]
pub struct LineIO {
  pub input:  Vec<String>,
  pub output: Vec<String>,
}

#[derive(Debug)]
pub enum Lobby {
  Start,
  End,
}
