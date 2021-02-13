use uuid::Uuid;
use std::collections::HashMap;
use std::error::Error;

use crate::session::Session;
use crate::server::Server;
use crate::message::{SessionAction, WorldAction, ServerAction, ServerEvent};

#[derive(Default)]
pub struct World {
  sessions: HashMap<Uuid,Session>,
}

impl World {
  pub fn new() -> Self {
    Default::default()
  }

  fn manage_session(&mut self, s: Session) {
    let id = s.id();
    self.sessions.insert(id, s);
    println!("managing session: {}", id);
  }
  fn drop_session(&mut self, id: Uuid) {
    self.sessions.remove(&id);
    println!("dropped session: {}", id);
  }
  /*
  pub fn get_session(&self, id: Uuid) -> Option<&Session> {
    self.sessions.get(&id)
  }
  pub fn get_session_mut(&mut self, id: Uuid) -> Option<&mut Session> {
    self.sessions.get_mut(&id)
  }
  */


  pub fn run(&mut self, mut server: Server) -> Result<(), Box<dyn Error>> {
    loop {

      // pump the server for events from connected clients
      for event in server.pump()? {
        match event {
          ServerEvent::Connect(sid) => {
            let s = Session::new(sid);
            self.manage_session(s);
          },
          ServerEvent::Disconnect(sid) => {
            self.drop_session(sid);
          },
          ServerEvent::Read(sid, str) => {
            if let Some(s) = self.sessions.get_mut(&sid) {
              s.queue_action(SessionAction::Input(str.to_string()));
            }
          },
        }
      }

      // run all the sessions, and collect up the changes they want to make to the world
      let mut world_actions = vec!();
      for (_, s) in self.sessions.iter_mut() {
        world_actions.append(&mut s.process_actions());
      }

      // apply actions to the world and collect up actions to take on connected clients
      let mut server_actions = vec!();
      for action in world_actions.iter() {
        match action {
          WorldAction::Wall(text) => {
            println!("wall: {}", text);
            for (id, _) in self.sessions.iter() {
              server_actions.push(ServerAction::Write(*id, text.to_string()));
            }
          },
        }
      }

      // apply actions to connected clients
      server.process_actions(server_actions)?;
    }
  }

}
