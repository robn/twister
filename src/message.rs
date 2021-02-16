use uuid::Uuid;

// things that happened in the server
#[derive(Debug)]
pub enum ServerEvent {
  Connect(Uuid),
  Disconnect(Uuid),
  Read(Uuid, String),
}

// things we want the world to do
#[derive(Debug)]
pub enum WorldAction {
  Wall(String),
}
