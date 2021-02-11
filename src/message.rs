use mio::Token;

// things that happened in the server
#[derive(Debug)]
pub enum ServerEvent {
  Connect(Token),
  Disconnect(Token),
  Read(Token, Vec<u8>),
}

// things we want the session to do
#[derive(Debug)]
pub enum SessionAction {
  Input(String),
}

// things we want the world to do
#[derive(Debug)]
pub enum WorldAction {
}

// things we want the server to do
#[derive(Debug)]
pub enum ServerAction {
  //Write(Token, Vec<u8>),
}
