use uuid::Uuid;

// things that happened in the server
#[derive(Debug)]
pub enum ServerEvent {
  Connect(Uuid),
  Disconnect(Uuid),
  Read(Uuid, Vec<u8>),
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
  //Write(Uuid, Vec<u8>),
}
